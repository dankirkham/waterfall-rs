use egui::*;
use egui_extras::image::RetainedImage;

use crate::configuration::Configuration;
use crate::waterfall_ticks::WaterfallTicks;

pub struct WaterfallPlot<'a> {
    image: &'a Option<RetainedImage>,
    config: &'a mut Configuration,
}

impl<'a> WaterfallPlot<'a> {
    pub fn new(image: &'a Option<RetainedImage>, config: &'a mut Configuration) -> Self {
        Self { image, config }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if let Some(image) = self.image {
            WaterfallTicks::new(self.config).ui(ui);

            egui::CentralPanel::default()
                .frame(Frame::none().fill(ui.style().visuals.faint_bg_color))
                .show_inside(ui, |ui| {
                    let size = ui.available_size();
                    let response = image.show_size(ui, size);
                    if response.hovered() {
                        if let Some(pos) = response.hover_pos() {
                            let interval_pos = pos.x / size.x;
                            let hover_freq = self.config.zoomed_interval_to_hz(interval_pos);
                            response.on_hover_text_at_pointer(hover_freq.to_string());
                        }
                    }
                });
        }
    }
}
