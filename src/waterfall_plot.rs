use egui::*;
use egui_extras::image::RetainedImage;

use crate::configuration::Configuration;
use crate::waterfall_ticks::WaterfallTicks;

pub struct WaterfallPlot<'a> {
    image: &'a Option<RetainedImage>,
    config: &'a Configuration,
}

impl<'a> WaterfallPlot<'a> {
    pub fn new(image: &'a Option<RetainedImage>, config: &'a Configuration) -> Self {
        Self { image, config }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if let Some(image) = self.image {
            WaterfallTicks::new(self.config).ui(ui);

            egui::CentralPanel::default()
                .frame(Frame::none())
                .show_inside(ui, |ui| {
                    let size = ui.available_size();
                    image.show_size(ui, size);
                });
        }
    }
}
