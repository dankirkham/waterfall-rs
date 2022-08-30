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
            ui.toggle_value(&mut self.show.settings, "🔧 Settings");
            ui.toggle_value(&mut self.show.scope, "🗠 Oscilloscope");
            ui.toggle_value(&mut self.show.messages, "📻 Messages");
            ui.toggle_value(&mut self.show.about, "❔ About");
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                global_dark_light_mode_switch(ui);
            });
        });
    }
}
