use std::f32::INFINITY;

use egui::Color32;
use rfd::FileDialog;

use crate::{mods::{self, scan_directory, Mod}, tools::text, ModManager};

pub fn settings_window(ctx: &egui::Context, show: &mut bool, game_dir: &mut String, mods_dir: &mut String, launch_options: &mut String) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(text("Settings", Color32::WHITE, true));

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // FIXME: Text not overflowing, but pushing other elements to the side
            ui.add_sized([350.0, 0.0], egui::TextEdit::singleline(game_dir).hint_text("Game Directory").desired_width(350.0));
            ui.add_space(3.0);
            if ui.button(text("...", Color32::WHITE, true)).clicked() {
                pick_executable(game_dir);
            };
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // FIXME: Text not overflowing, but pushing other elements to the side
            ui.add_sized([350.0, 0.0], egui::TextEdit::singleline(mods_dir).hint_text("Mods Scan Directory").desired_width(INFINITY));
            ui.add_space(3.0);
            if ui.button(text("...", Color32::WHITE, true)).clicked() {
                pick_folder(mods_dir);
            };
        });

        ui.add_space(10.0);

        ui.add(egui::TextEdit::singleline(launch_options).hint_text("Executable launch options"));

        ui.add_space(10.0);

        if ui.button("Return").clicked() {
            *show = false;
        };
    });
}

fn pick_executable(game_dir: &mut String) {
    if let Some(path) = FileDialog::new().add_filter("Executable", &["exe"]).pick_file() {
        *game_dir = path.display().to_string();
    }
}

fn pick_folder(mods_dir: &mut String) {
    if let Some(path) = FileDialog::new().pick_folder() {
        *mods_dir = path.display().to_string();
        scan_directory(mods_dir);
    }
}