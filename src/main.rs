use eframe::{egui, App, CreationContext};
use rfd::FileDialog;

struct CatDogApp {
    selected_image: Option<String>,
}

impl Default for CatDogApp {
    fn default() -> Self {
        Self {
            selected_image: None,
        }
    }
}

impl App for CatDogApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select a cat or dog image");

            if ui.button("Select Image").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Image", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    self.selected_image = Some(path.display().to_string());
                }
            }

            if let Some(ref path) = self.selected_image {
                ui.label(format!("You choose the picture: {}", path));
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cat vs Dog Classifier",
        options,
        Box::new(|_cc: &CreationContext| Box::new(CatDogApp::default())),
    )
}
