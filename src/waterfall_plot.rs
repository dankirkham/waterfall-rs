use egui::*;
use egui_extras::image::RetainedImage;

pub struct WaterfallPlot<'a> {
    image: &'a Option<ColorImage>,
}

impl<'a> WaterfallPlot<'a> {
    pub fn new(image: &'a Option<ColorImage>) -> Self {
        Self {
            image,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if let Some(image) = self.image {
            let size = ui.available_size();
            let display_image = RetainedImage::from_color_image("waterfall-image", image.clone());
            display_image.show_size(ui, size);
        }
    }
}
