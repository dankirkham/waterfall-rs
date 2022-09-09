use egui::*;
use crate::configuration::{AxisMode, Configuration, TriggerMode};

pub struct Settings<'a> {
    config: &'a mut Configuration,
}

impl<'a> Settings<'a> {
    pub fn new(config: &'a mut Configuration) -> Self {
        Self { config }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            egui::ComboBox::from_label("X-Axis Mode")
                .selected_text(format!("{:?}", self.config.scope.x_mode))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.scope.x_mode, AxisMode::Fit, "Fit");
                });
            egui::ComboBox::from_label("Y-Axis Mode")
                .selected_text(format!("{:?}", self.config.scope.y_mode))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.scope.y_mode, AxisMode::Fit, "Fit");
                });
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.heading("Trigger");
            });
            egui::ComboBox::from_label("Triger Mode")
                .selected_text(format!("{:?}", self.config.scope.trigger.mode))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.config.scope.trigger.mode,
                        TriggerMode::Auto,
                        "Auto",
                    );
                    ui.selectable_value(
                        &mut self.config.scope.trigger.mode,
                        TriggerMode::Rising,
                        "Rising",
                    );
                    ui.selectable_value(
                        &mut self.config.scope.trigger.mode,
                        TriggerMode::Falling,
                        "Falling",
                    );
                });
            if self.config.scope.trigger.mode != TriggerMode::Auto {
                egui::Grid::new("trigger-level").show(ui, |ui| {
                    ui.add(
                        egui::DragValue::new(&mut self.config.scope.trigger.level)
                            .speed(0.00000001), // .custom_formatter(|n, _| format!("{}", Frequency::Hertz(n)))
                    );
                    ui.label("Level");
                    ui.end_row();
                });
            }
        });
    }
}
