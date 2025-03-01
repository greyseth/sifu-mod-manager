use egui::{style::Interaction, Color32};
use mods::Mod;
use tools::text;
use settings::settings_window;

mod tools;
mod mods;
mod settings;

struct ModManager {
    mod_count: usize,
    mods: Vec<Mod>,
    open_settings: bool
}

impl Default for ModManager {
    fn default() -> Self {
        let mut new_mods = Vec::new();
        new_mods.push(Mod::new("Mod 1".to_string(), 100000));
        new_mods.push(Mod::new("Mod 2".to_string(), 10000));
        new_mods.push(Mod::new("Mod 3".to_string(), 15000));

        Self {
           mod_count: 0,
           mods: new_mods,
           open_settings: false
        }
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
                            self.mod_count = 10;
                        }
                        if ui.button("Enable all").clicked() {
                            println!("Enabling all mods...");
                        }
                        if ui.button("Disable all").clicked() {
                            println!("Disabling all mods...");
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
                                        ui.label(m.size.to_string());
                                    });
                                });
                                
                                response.response
                            }).response.interact(egui::Sense::click());

                        if response.clicked() {
                            m.enabled = !m.enabled;
                        }
                    }
                });


                egui::Frame::new().show(ui, |ui| {
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            let launch_options = ui.add(egui::TextEdit::singleline(&mut String::new()).hint_text("Executable launch options"));
                            ui.button("Launch Game");
                        });
                        ui.add_space(10.0);
                        ui.button("Apply changes");
                    });
                });
            });
        });

        if self.open_settings {settings_window(ctx, &mut self.open_settings);}
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 800.0]), ..Default::default()
    };

    eframe::run_native("Sifu Mod Manager", options, Box::new(|_cc| Ok(Box::<ModManager>::default())))
}
