use egui::*;
use egui_extras::image::RetainedImage;

pub struct WaterfallPlot<'a> {
    image: &'a Option<RetainedImage>,
}

impl<'a> WaterfallPlot<'a> {
    pub fn new(image: &'a Option<RetainedImage>) -> Self {
        Self {
            image,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if let Some(image) = self.image {
            let size = ui.available_size();
            image.show_size(ui, size);
        }
    }
}
