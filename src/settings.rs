use egui::Color32;

use crate::tools::text;

pub fn settings_window(ctx: &egui::Context, show: &mut bool) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(text("Settings", Color32::WHITE, true));

        ui.add_space(10.0);

        if ui.button("Return").clicked() {
            *show = false;
        };
    });
}