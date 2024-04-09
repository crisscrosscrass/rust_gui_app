use eframe::{ run_native, App, NativeOptions, Frame };
use egui::{ Context, DroppedFile, ViewportBuilder };

mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([640.0, 240.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    run_native("Sample Program", options, Box::new(|_cc| Box::<ExampleApp>::default()))
}

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