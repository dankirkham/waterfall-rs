use crate::configuration::{Configuration, ScopeMode};
use egui::*;

pub struct Buttons<'a> {
    config: &'a mut Configuration,
}

impl<'a> Buttons<'a> {
    pub fn new(config: &'a mut Configuration) -> Self {
        Self { config }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                let red = if ui.style().visuals.dark_mode {
                    Color32::DARK_RED
                } else {
                    Color32::LIGHT_RED
                };

                let green = if ui.style().visuals.dark_mode {
                    Color32::DARK_GREEN
                } else {
                    Color32::LIGHT_GREEN
                };

                if ui
                    .add(match self.config.scope.mode {
                        ScopeMode::Stop => Button::new("Stop").fill(red),
                        _ => Button::new("Stop"),
                    })
                    .clicked()
                {
                    self.config.scope.mode = ScopeMode::Stop;
                }

                if ui
                    .add(match self.config.scope.mode {
                        ScopeMode::Single => Button::new("Single").fill(green),
                        _ => Button::new("Single"),
                    })
                    .clicked()
                {
                    self.config.scope.mode = ScopeMode::Single;
                }

                if ui
                    .add(match self.config.scope.mode {
                        ScopeMode::Run => Button::new("Run").fill(green),
                        _ => Button::new("Run"),
                    })
                    .clicked()
                {
                    self.config.scope.mode = ScopeMode::Run;
                }
            });
        });
    }
}
