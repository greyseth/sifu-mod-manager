use std::{fs::{self, create_dir, create_dir_all, FileType}, path::Path};

use serde::{Deserialize, Serialize};

use crate::{settings::save_settings, tools::{copy_dir_all, remove_dir}, ModManager};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mod {
    pub enabled: bool,
    pub name: String,
    pub size: usize,
}

impl Mod {
    pub fn new(name: String, size: usize, enabled: bool) -> Self {
        Mod {name, size, enabled}
    }
}

fn is_enabled(mods: &Vec<Mod>, file_name: String) -> bool {
    let mut return_bool = false;

    for m in mods.iter() {
        if m.name == file_name && m.enabled {return_bool = true;}
    }

    return_bool
}

pub fn scan_directory(mods_dir: &String, mods: &mut Vec<Mod>) -> Vec<Mod> {
    let mut scanned_mods = Vec::new();
    
    let path = std::path::Path::new(mods_dir);

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            scanned_mods.push(Mod::new(
                                entry.file_name().to_string_lossy().to_string(),
                                fs::read_dir(path)
                                    .ok()
                                    .into_iter()
                                    .flat_map(|entries| entries.flatten())
                                    .filter_map(|entry| entry.metadata().ok())
                                    .map(|metadata| metadata.len() as usize).sum(),
                                is_enabled(&mods, entry.file_name().to_string_lossy().to_string())
                            ));
                        }else {
                            let file_name = entry.file_name().to_string_lossy().to_string();
                            if file_name.ends_with("zip") || file_name.ends_with("rar") || file_name.ends_with("7z") {
                                scanned_mods.push(
                                    Mod::new(
                                        file_name, 
                                        metadata.len() as usize, 
                                        is_enabled(&mods, entry.file_name().to_string_lossy().to_string()))
                                );
                            }
                        }
                    }
                }
            }
        }else {msgbox::create("Couldn't scan mods", "Failed to read contents in mods folder", msgbox::IconType::Error);}
    }else {msgbox::create("Invalid path", "The provided mods scanning path is invalid", msgbox::IconType::Error);}
    
    scanned_mods
}

pub fn clear_mods(game_dir: &str) {
    let game_path = Path::new(game_dir).parent();
    if let Some(path) = game_path {
        let mods_path = path.join("Sifu/Content/Paks/~mods");
        if mods_path.exists() {
            remove_dir(&mods_path);
        }else {
            create_dir_all(mods_path);
        }
    }
}

pub fn apply_mods(mod_manager: &mut ModManager) {
    clear_mods(&mod_manager.game_dir);

    let game_path = Path::new(&mod_manager.game_dir).parent();
    if let Some(path) = game_path {
        let enabled_mods: Vec<Mod> = mod_manager.mods.iter().filter(|m| m.enabled).cloned().collect();
        for m in enabled_mods.iter() {
            let mod_path = Path::new(&mod_manager.mods_dir).join(&m.name);
            if mod_path.is_dir() {
                let copied_files = copy_dir_all(mod_path, path.join("Sifu/Content/Paks/~mods"));
                println!("{:?}", copied_files);
                match copied_files {
                    Ok(()) => msgbox::create("Applied selected mods", "Mods successfully applied", msgbox::IconType::Info),
                    Err(err) => msgbox::create("Failed to apply mods", "An error has occurred", msgbox::IconType::Error),
                };
            }else {
                // TODO: Implement zip extraction
            }
        }
    }

    save_settings(mod_manager);
}