use std::{fs, io, path::Path};

pub fn text(text: &str, color: egui::Color32, is_heading: bool) -> egui::RichText {
    let mut rich_text = egui::RichText::new(text).color(color);
    if is_heading {rich_text = rich_text.heading();}
    rich_text
}

pub fn remove_dir(path: &Path) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        remove_dir(&entry.path());
                        fs::remove_dir(entry.path());
                    }else {
                        fs::remove_file(entry.path());
                    }
                }
            }
        }
    }
}

// Thanks, guy from StackOverflow 👍
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn load_icon() -> egui::IconData {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("icon.png");
		let image = image::load_from_memory(icon)
			.expect("Failed to open icon path")
			.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	
	egui::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	}
}