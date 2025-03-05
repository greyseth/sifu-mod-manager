use std::{env::current_exe, f32::INFINITY, fs, path::Path};

use eframe::glow::EXTENSIONS;
use egui::Color32;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

use crate::{mods::{self, scan_directory, Mod}, tools::text, ModManager, Platform};

pub fn settings_window(ctx: &egui::Context, mod_manager: &mut ModManager) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(text("Settings", Color32::WHITE, true));

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // FIXME: Text not overflowing, but pushing other elements to the side
            ui.add_sized([350.0, 0.0], egui::TextEdit::singleline(&mut mod_manager.game_dir).hint_text("Game Directory").desired_width(350.0));
            ui.add_space(3.0);
            if ui.button(text("...", Color32::WHITE, true)).clicked() {
                pick_executable(&mut mod_manager.game_dir, &mut mod_manager.game_dir_valid);
            };

            ui.label(text(if mod_manager.game_dir_valid {"Game directory valid"}else {"Invalid game directory"}, if mod_manager.game_dir_valid {Color32::LIGHT_GREEN}else{Color32::LIGHT_RED}, true));
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // FIXME: Text not overflowing, but pushing other elements to the side
            ui.add_sized([350.0, 0.0], egui::TextEdit::singleline(&mut mod_manager.mods_dir).hint_text("Mods Scan Directory").desired_width(INFINITY));
            ui.add_space(3.0);
            if ui.button(text("...", Color32::WHITE, true)).clicked() {
                pick_folder(&mut mod_manager.mods_dir, &mut mod_manager.mods);
            };

            // FIXME: Refreshing doesn't work on settings page
            if ui.button("Refresh mod list").clicked() {scan_directory(&mod_manager.mods_dir, &mut mod_manager.mods); println!("{:?}", mod_manager.mods);}
        });

        ui.add_space(10.0);

        ui.add(egui::TextEdit::singleline(&mut mod_manager.launch_options).hint_text("Executable launch options"));

        ui.add_space(10.0);

        egui::ComboBox::from_label("Select Platform").selected_text(mod_manager.platform.to_string()).show_ui(ui, |ui| {
            ui.selectable_value(&mut mod_manager.platform, Platform::Epic, Platform::Epic.to_string());
            ui.selectable_value(&mut mod_manager.platform, Platform::Steam, Platform::Steam.to_string());
            ui.selectable_value(&mut mod_manager.platform, Platform::Direct, Platform::Direct.to_string());
        });

        ui.add_space(10.0);

        if ui.button("Return").clicked() {
            mod_manager.open_settings = false;
            save_settings(mod_manager);
        };
    });
}

fn pick_executable(game_dir: &mut String, game_dir_valid: &mut bool) {
    if let Some(path) = FileDialog::new().add_filter("Executable", &["exe"]).pick_file() {
        *game_dir = path.display().to_string();

        // Verifies executable path
        if let Some(dir) = path.as_path().parent() {
            let paks_folder = dir.join("Sifu/Content/Paks");
            *game_dir_valid = paks_folder.exists() && paks_folder.is_dir();
        }else {*game_dir_valid = false;}
    }
}

fn pick_folder(mods_dir: &mut String, mods: &mut Vec<Mod>) {
    if let Some(path) = FileDialog::new().pick_folder() {
        *mods_dir = path.display().to_string();
        // FIXME: Refreshing doesn't work on settings page
        scan_directory(&path.display().to_string(), mods);
    }
}

pub fn save_settings(mod_manager: &ModManager) {
    if let Ok(exe_dir) = current_exe() {
        if let Some(path) = exe_dir.parent() {
            let pathname = path.join("settings.toml").to_path_buf().to_string_lossy().to_string();
            
            let serialized = toml::to_string(mod_manager).expect("Failed to serialize settings");
            if let Err(error) = fs::write(pathname, serialized) {
                msgbox::create("Failed to save", "Failed to save settings", msgbox::IconType::Error);
            }
        }else {msgbox::create("Failed to save", "Failed to save settings", msgbox::IconType::Error);}
    }
}