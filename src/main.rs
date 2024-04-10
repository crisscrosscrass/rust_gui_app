use eframe::{ run_native, App, NativeOptions, Frame };
use egui::{ Context, DroppedFile, ViewportBuilder, IconData };

mod ui;

pub struct ExampleApp {
    dropped_files: Vec<DroppedFile>,
    picked_path: Option<String>,
    input_text: String,
}


impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            dropped_files: Vec::new(),
            picked_path: None,
            input_text: String::new(),
        }
    }
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ui::update_ui(ctx, self);
    }
}

fn main() -> Result<(), eframe::Error> {
    let icon_image =image::open("./assets/LogoApp.png").unwrap();
let width = icon_image.width();
let height = icon_image.height();
let icon_rgba8 = icon_image.into_rgba8().to_vec();
let icon_data =IconData{
         rgba: icon_rgba8,
         width,
         height,
};
    let options = NativeOptions {
        centered: true,
        viewport: ViewportBuilder::default()
            .with_icon(icon_data)
            .with_inner_size([1200.0, 480.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    run_native("Sample Program", options, Box::new(|cc| {
        // This gives us image support:
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Box::<ExampleApp>::default()
    }),)
} 