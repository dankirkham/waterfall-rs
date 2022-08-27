use egui::*;

use crate::ui::Windows;

pub struct Toolbar<'a> {
    show: &'a mut Windows,
}

impl<'a> Toolbar<'a> {
    pub fn new(show: &'a mut Windows) -> Self {
        Self { show }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.toggle_value(&mut self.show.settings, "⚙️ Settings");
            ui.toggle_value(&mut self.show.scope, "🗠 Scope");
        });
    }
}
