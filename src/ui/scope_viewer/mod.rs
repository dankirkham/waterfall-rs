mod buttons;
mod settings;

use egui::plot::{Line, Plot, Values};

use crate::configuration::Configuration;
use crate::types::SampleType;
use buttons::Buttons;
use settings::Settings;

pub struct ScopeViewer<'a> {
    plot_data: &'a [SampleType],
    config: &'a mut Configuration,
}

impl<'a> ScopeViewer<'a> {
    pub fn new(config: &'a mut Configuration, plot_data: &'a [SampleType]) -> Self {
        Self { config, plot_data }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let mut buttons = Buttons::new(self.config);
        buttons.ui(ui);

        let line = Line::new(Values::from_ys_f32(self.plot_data));
        Plot::new("Scope")
            // .view_aspect(1.732)
            .center_y_axis(true)
            // .include_y(1.0)
            // .include_y(-1.0)
            // .include_x(450.0)
            .height(300.0)
            // .width(400.0)
            .show(ui, |plot_ui| plot_ui.line(line));

        let mut settings = Settings::new(self.config);
        settings.ui(ui);
    }
}
