use eframe::{
    egui::{self, ColorImage, TextureHandle, Image},
    App,
};
use rfd::FileDialog;
use image::ImageReader;

pub struct CatDogApp {
    selected_path: Option<String>,
    texture: Option<TextureHandle>,
}

impl Default for CatDogApp {
    fn default() -> Self {
        Self {
            selected_path: None,
            texture: None,
        }
    }
}

impl App for CatDogApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Cat vs Dog Classifier");

            if ui.button("Select Image").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Image", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    self.selected_path = Some(path.display().to_string());

                    if let Ok(reader) = ImageReader::open(&path) {
                        if let Ok(img) = reader.decode() {
                            let size = [img.width() as usize, img.height() as usize];
                            let img = img.to_rgba8();
                            let pixels = img.as_flat_samples();
                            let color_image = ColorImage::from_rgba_unmultiplied(
                                size,
                                pixels.as_slice(),
                            );
                            self.texture = Some(
                                ctx.load_texture("selected_image", color_image, Default::default()),
                            );
                        }
                    }
                }
            }

            if let Some(path) = &self.selected_path {
                ui.label(format!("Selected: {}", path));
            }

            if let Some(texture) = &self.texture {
                let available_width = ui.available_width();
                let scale = available_width / (texture.size()[0] as f32);
                let desired_size = egui::vec2(
                    texture.size()[0] as f32 * scale,
                    texture.size()[1] as f32 * scale,
                );

                ui.add(
                    Image::from(texture)
                        .fit_to_exact_size(desired_size)
                        .sense(egui::Sense::hover()),
                );
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cat vs Dog Classifier",
        options,
        Box::new(|_cc| Box::new(CatDogApp::default())),
    )
}
