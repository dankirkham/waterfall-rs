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
            if ui.add(if self.show.settings {
                egui::Button::new("Settings")
                    .fill(ui.style().visuals.faint_bg_color)
            } else {
                egui::Button::new("Settings")
            }).clicked() {
                self.show.settings = !self.show.settings;
            }

            if ui.add(if self.show.scope {
                egui::Button::new("Scope")
                    .fill(ui.style().visuals.faint_bg_color)
            } else {
                egui::Button::new("Scope")
            }).clicked() {
                self.show.scope = !self.show.scope;
            }
        });
    }
}
