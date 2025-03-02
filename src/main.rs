use std::{env::current_exe, fs, path::{Path, PathBuf}};

use eframe::glow::NUM_EXTENSIONS;
use egui::Color32;
use mods::{scan_directory, Mod};
use serde::{Deserialize, Serialize};
use tools::text;
use settings::{save_settings, settings_window};

mod tools;
mod mods;
mod settings;

#[derive(Serialize, Deserialize)]
struct ModManager {
    mods: Vec<Mod>,
    open_settings: bool,
    game_dir: String,
    game_dir_valid: bool,
    mods_dir: String,
    launch_options: String
}

impl Default for ModManager {
    fn default() -> Self {
        // This part really sucks...
        let mut exe = PathBuf::new();
        let mut path= Path::new("");
        let pathname = if let Ok(_exe) = current_exe() {
            exe = _exe;
            if let Some(_path) = exe.parent() {
                path = &_path;
                _path.to_path_buf().to_string_lossy().to_string()
            }else {"".to_string()}
        }else {"".to_string()};
        
        let mut return_self = Self {
           mods: scan_directory(&pathname),
           open_settings: false,
           game_dir: "".to_string(),
           mods_dir: pathname,
           launch_options: "".to_string(),
           game_dir_valid: false
        };
        
        // Reads from settings.toml
        if path.join("settings.toml").exists() {
            let settings_content = fs::read_to_string(path.join("settings.toml")).expect("Failed to read settings file");
            return_self = toml::from_str(&settings_content).ok().unwrap();
        }
        
        return_self
    }
}

impl eframe::App for ModManager {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(text("Sifu Mod Manager", Color32::WHITE, true));

                egui::Frame::new().outer_margin(egui::Margin {bottom: 16, ..Default::default()}).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Refresh mods").clicked() {
                            if self.mods_dir.is_empty() {msgbox::create("No mods directory set", "Please set the mods scanning directory in the settings", msgbox::IconType::Error);}
                            else {self.mods = scan_directory(&self.mods_dir);}
                        }
                        if ui.button("Enable all").clicked() {
                            for m in self.mods.iter_mut() {
                                m.enabled = true;
                            }
                        }
                        if ui.button("Disable all").clicked() {
                            for m in self.mods.iter_mut() {
                                m.enabled = false;
                            }
                        }
                        if ui.button("Settings").clicked() {
                            self.open_settings = true;
                        }
                    });
                });

                egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for m in &mut self.mods {
                        let response = egui::Frame::new()
                            .fill(Color32::BLACK)
                            .corner_radius(egui::CornerRadius::same(5))
                            .inner_margin(egui::Margin::same(5))
                            .show(ui, |ui| {
                                let response = ui.horizontal(|ui| {
                                    ui.checkbox(&mut m.enabled, "");
                                    ui.label(text(format!("{}", m.name).as_str(), Color32::WHITE, false));

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(format!("{:.2} KB", (m.size as f64 / 1024.0).to_string()))
                                    });
                                });
                                
                                response.response
                            }).response.interact(egui::Sense::click());

                        if response.clicked() {
                            m.enabled = !m.enabled;
                        }
                    }
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.button(text("Apply Changes", Color32::WHITE, true));
                        if ui.button(text("Launch Game", Color32::WHITE, true)).clicked() {
                            if self.game_dir.is_empty() {msgbox::create("No directory set", "Please set game directory in the settings", msgbox::IconType::Error);}
                            else {
                                if !self.game_dir_valid {msgbox::create("Invalid game path", "Please enter a valid path to Sifu.exe", msgbox::IconType::Error);}
                                else {save_settings(&self);}
                            }
                        };
                    });
                });
            });
        });

        if self.open_settings {settings_window(ctx, self);}
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 800.0]), ..Default::default()
    };

    eframe::run_native("Sifu Mod Manager", options, Box::new(|_cc| Ok(Box::<ModManager>::default())))
}
