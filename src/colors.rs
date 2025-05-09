use egui::Color32;

pub const SUCCESS: Color32 = Color32::GREEN;
pub const WARN: Color32 = Color32::ORANGE;

pub const GRAY_DARK: Color32 = Color32::from_gray(47);
pub const GRAY: Color32 = Color32::from_gray(169);

pub fn get_text_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.visuals().dark_mode {
        egui::Color32::WHITE
    } else {
        egui::Color32::BLACK
    }
}
