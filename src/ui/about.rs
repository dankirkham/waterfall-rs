use egui::*;

pub struct About;

impl About {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("waterfall-rs");
        ui.label("A waterfall and SDR tool written in Rust.");
        ui.hyperlink_to("github", "https://github.com/dankirkham/waterfall-rs");
    }
}
