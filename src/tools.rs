pub fn text(text: &str, color: egui::Color32, is_heading: bool) -> egui::RichText {
    let mut rich_text = egui::RichText::new(text).color(color);
    if is_heading {rich_text = rich_text.heading();}
    rich_text
}