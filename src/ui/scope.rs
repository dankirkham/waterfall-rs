use egui::*;
use egui::plot::{Line, Plot, Values};

use crate::configuration::{AxisMode, Configuration, ScopeMode, TriggerMode, TriggerSettings};
use crate::types::SampleType;

pub struct Scope<'a> {
    plot_data: &'a Vec<SampleType>,
    config: &'a mut Configuration,
}

fn trigger_position(data: &[SampleType], settings: &TriggerSettings) -> Option<usize> {
    let TriggerSettings { mode, level } = settings;
    let lower = |d| d < level;
    let higher = |d| d > level;
    match mode {
        TriggerMode::Rising => {
            let below = data.iter().position(lower)?;
            data[below..].iter().position(higher)
        },
        TriggerMode::Falling => {
            let above = data.iter().position(higher)?;
            data[above..].iter().position(lower)
        },
        TriggerMode::Auto => Some(0),
    }
}

impl<'a> Scope<'a> {
    pub fn new(config: &'a mut Configuration, plot_data: &'a Vec<SampleType>) -> Self {
        Self { config, plot_data }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let plot_data = match trigger_position(self.plot_data, &self.config.scope.trigger) {
            Some(start_pos) => &self.plot_data[start_pos..],
            None => &[],
        };

        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                if ui.add(match self.config.scope.mode {
                    ScopeMode::Stop => Button::new( "Stop")
                        .fill(Color32::DARK_RED),
                    _ => Button::new( "Stop")
                }).clicked() {
                    self.config.scope.mode = ScopeMode::Stop;
                }

                if ui.add(match self.config.scope.mode {
                    ScopeMode::Single => Button::new( "Single")
                        .fill(Color32::DARK_GREEN),
                    _ => Button::new( "Single")
                }).clicked() {
                    self.config.scope.mode = ScopeMode::Single;
                }

                if ui.add(match self.config.scope.mode {
                    ScopeMode::Run => Button::new( "Run")
                        .fill(Color32::DARK_GREEN),
                    _ => Button::new( "Run")
                }).clicked() {
                    self.config.scope.mode = ScopeMode::Run;
                }
            });
        });

        let line = Line::new(Values::from_ys_f32(plot_data));
        Plot::new("Scope")
            // .view_aspect(1.732)
            .center_y_axis(true)
            // .include_y(1.0)
            // .include_y(-1.0)
            .include_x(450.0)
            .height(300.0)
            // .width(400.0)
            .show(ui, |plot_ui| plot_ui.line(line));

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
                    ui.selectable_value(&mut self.config.scope.trigger.mode, TriggerMode::Auto, "Auto");
                    ui.selectable_value(&mut self.config.scope.trigger.mode, TriggerMode::Rising, "Rising");
                    ui.selectable_value(&mut self.config.scope.trigger.mode, TriggerMode::Falling, "Falling");
                });
                if self.config.scope.trigger.mode != TriggerMode::Auto {
                    egui::Grid::new("trigger-level").show(ui, |ui| {
                        ui.add(
                            egui::DragValue::new(&mut self.config.scope.trigger.level)
                                .speed(0.00000001)
                                // .custom_formatter(|n, _| format!("{}", Frequency::Hertz(n)))
                        );
                        ui.label("Level");
                        ui.end_row();
                    });
                }
        });
    }
}
