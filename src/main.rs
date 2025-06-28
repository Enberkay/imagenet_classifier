use eframe::{
    egui::{self, ColorImage},
    App,
};
use image::io::Reader as ImageReader;
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use ndarray::{Array, Ix4};
use ort::{
    session::{ Session},
    value::Value,
};
use rfd::FileDialog;
use serde_json;
use std::fs;

struct CatDogApp {
    texture: Option<egui::TextureHandle>,
    prediction: Option<String>,
    session: Session,
    labels: Vec<String>,
}

impl CatDogApp {
    fn load_labels(path: &str) -> Vec<String> {
        let data = fs::read_to_string(path).expect("Cannot read labels file");
        serde_json::from_str::<Vec<String>>(&data).expect("Cannot parse labels JSON")
    }

    fn prepare_image(img: &DynamicImage) -> Array<f32, Ix4> {
        let resized = img.resize_exact(224, 224, FilterType::CatmullRom);

        let mut data = Vec::with_capacity(224 * 224 * 3);
        for y in 0..224 {
            for x in 0..224 {
                let pixel = resized.get_pixel(x, y);
                data.push(pixel[0] as f32 / 255.0);
                data.push(pixel[1] as f32 / 255.0);
                data.push(pixel[2] as f32 / 255.0);
            }
        }

        Array::from_shape_fn((1, 3, 224, 224), |(_, c, y, x)| data[(y * 224 + x) * 3 + c])
    }

    fn run_inference(&mut self, input: &Array<f32, Ix4>) -> Option<String> {
        // สร้าง Value จาก ndarray โดยตรง
        let input_tensor = Value::from_array(input.clone()).ok()?;

        let input_name = self.session.inputs[0].name.clone();
        let inputs = vec![(input_name.as_str(), &input_tensor)];

        let outputs = self.session.run(inputs).ok()?;

        // ดึง output เป็น slice ของ f32
        let (_, output_data): (&ort::tensor::Shape, &[f32]) = outputs[0].try_extract_tensor().ok()?;

        // หา index ที่มีค่ามากที่สุด
        let max_idx = output_data
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)?;

        self.labels.get(max_idx).cloned()
    }
}

impl Default for CatDogApp {
    fn default() -> Self {
        // สร้าง session จาก model file
        let session = Session::builder()
            .unwrap()
            .commit_from_file("mobilenetv2-7.onnx")
            .unwrap();

        let labels = CatDogApp::load_labels("imagenet-simple-labels.json");

        Self {
            texture: None,
            prediction: None,
            session,
            labels,
        }
    }
}

impl App for CatDogApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ImageNet Classifier");

            if ui.button("Select Image").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Image", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    if let Ok(reader) = ImageReader::open(&path) {
                        if let Ok(img) = reader.decode() {
                            let size = [img.width() as usize, img.height() as usize];
                            let img_rgba = img.to_rgba8();
                            let pixels = img_rgba.as_flat_samples();
                            let color_image =
                                ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

                            self.texture = Some(ctx.load_texture(
                                "selected_image",
                                color_image,
                                Default::default(),
                            ));

                            let input_tensor = CatDogApp::prepare_image(&img);
                            self.prediction = self.run_inference(&input_tensor);
                        }
                    }
                }
            }

            if let Some(texture) = &self.texture {
                // จำกัดขนาดรูปให้ไม่ใหญ่เกินไป
                let max_size = 400.0;
                let texture_size = texture.size_vec2();
                let scale = (max_size / texture_size.x.max(texture_size.y)).min(1.0);
                let display_size = texture_size * scale;
                
                ui.image(texture, display_size);
            }

            if let Some(pred) = &self.prediction {
                ui.separator();
                ui.label(format!("Prediction: {}", pred));
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ImageNet Classifier",
        options,
        Box::new(|_cc| Box::new(CatDogApp::default())),
    )
}
