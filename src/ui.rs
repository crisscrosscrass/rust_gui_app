use egui::{menu, CentralPanel, Context, DroppedFile, SidePanel, TopBottomPanel };

use super::ExampleApp;

pub fn update_ui(ctx: &Context, app: &mut ExampleApp) {
    draw_top_panel(ctx, &mut app.picked_path);
    draw_central_panel(ctx, &mut app.input_text);
    draw_right_panel(ctx, &mut app.dropped_files, &mut app.picked_path);
    preview_files_being_dropped(ctx);
}

pub fn draw_central_panel(ctx: &Context, input_text: &mut String) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Main Area");
        ui.horizontal(|ui| {
            ui.label(format!("Input Field: "));
            ui.separator();
            ui.text_edit_singleline(input_text);
        });
        ui.horizontal(|ui| {
            ui.label(format!("Ausgabe: "));
            ui.separator();
            ui.label(input_text.clone());
        });
        ui.horizontal(|ui| {
            if ui.button("Submit").clicked() {
                println!("Submitted text: {}", input_text);
            }
            if ui.button("change Input to 'Ace'").clicked() {
                *input_text = String::from("Ace");
            }
        });
    });
}

pub fn draw_top_panel(ctx: &Context, picked_path: &mut Option<String>) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        *picked_path = Some(path.display().to_string());
                    }
                }
                ui.button("Save").clicked();
                ui.separator();
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Edit", |ui| {
                ui.button("Copy").clicked();
                ui.button("Paste").clicked();
            });
            ui.menu_button("Help", |ui| {
                ui.button("About").clicked();
            });
        });
    });
}



pub fn draw_right_panel(ctx: &Context, dropped_files: &mut Vec<DroppedFile>, picked_path: &mut Option<String>) {
    SidePanel::right("my_right_panel").show(ctx, |ui| {
        ui.heading("Rechte Seite");
        ui.label("Drag-and-drop test!");

        if ui.button("Open file…").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                *picked_path = Some(path.display().to_string());
            }
        }

        if let Some(picked_path) = picked_path {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(picked_path);
            });
        }

        if !dropped_files.is_empty() {
            ui.group(|ui| {
                ui.label("Dropped files:");

                for file in dropped_files {
                    let mut info = if let Some(path) = &file.path {
                        path.display().to_string()
                    } else if !file.name.is_empty() {
                        file.name.clone()
                    } else {
                        "???".to_owned()
                    };

                    let mut additional_info = vec![];
                    if !file.mime.is_empty() {
                        additional_info.push(format!("type: {}", file.mime));
                    }
                    if let Some(bytes) = &file.bytes {
                        additional_info.push(format!("{} bytes", bytes.len()));
                    }
                    if !additional_info.is_empty() {
                        info += &format!(" ({})", additional_info.join(", "));
                    }

                    ui.label(info);
                }
            });
        }
    });
}

// https://github.com/woelper/egui_pick_file:
/// Preview hovering files:
pub fn preview_files_being_dropped(ctx: &Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}