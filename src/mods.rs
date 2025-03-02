use std::fs::{self, FileType};

pub struct Mod {
    pub enabled: bool,
    pub name: String,
    pub size: usize,
}

impl Mod {
    pub fn new(name: String, size: usize) -> Self {
        Mod {enabled: false, name, size}
    }
}

pub fn scan_directory(mods_dir: &String) -> Vec<Mod> {
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
                                    .map(|metadata| metadata.len() as usize).sum()
                            ));
                        }else {
                            // FIXME: path.extension() returning None for some reason
                            if let Some(extension) = path.extension() {
                                if extension == "zip" || extension == "rar" {
                                    scanned_mods.push(Mod::new(
                                        entry.file_name().to_string_lossy().to_string(), 
                                    metadata.len() as usize));
                                }
                            }
                        }
                    }
                }
            }
        }else {msgbox::create("Couldn't scan mods", "Failed to read contents in mods folder", msgbox::IconType::Error);}
    }else {msgbox::create("Invalid path", "The provided mods scanning path is invalid", msgbox::IconType::Error);}
    
    scanned_mods
}