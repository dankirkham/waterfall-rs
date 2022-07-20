use std::sync::mpsc::Receiver;
use std::ops::IndexMut;

use egui::*;
use egui_extras::image::RetainedImage;

use crate::plot_data::{PlotData, PlotRow, PLOT_DEPTH};
use crate::turbo::get_color;

pub struct WaterfallPlot<'a> {
    plot_data: &'a mut PlotData,
    plot_row_rx: &'a mut Receiver<PlotRow>,
}

impl<'a> WaterfallPlot<'a> {
    pub fn new(plot_row_rx: &'a mut Receiver<PlotRow>, plot_data: &'a mut  PlotData) -> Self {
        Self { plot_row_rx, plot_data }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        while let Ok(row) = self.plot_row_rx.try_recv() {
            *self.plot_data.push_back() = row;
        }

        let data = &self.plot_data;
        let data_height = data.len();
        if data_height == 0 {
            return;
        }

        // TODO: Use spectrum width
        let data_width = data[0].len();
        if data_width == 0 {
            return;
        }

        let mut image = ColorImage::new([data_width, PLOT_DEPTH], Color32::default());
        for y in 0..data_height {
            let row = &data[y];
            // let offset = y * data_width;
            for x in 0..data_width {
                let sample = row[x];
                let [red, green, blue] = get_color(sample.into());
                let color = Color32::from_rgb(red, green, blue);
                let x_min = x as usize;
                let y_min = (y + PLOT_DEPTH - data_height) as usize;

                *image.index_mut((x_min, y_min)) = color;

            }
        }

        let size = ui.available_size();

        let display_image = RetainedImage::from_color_image(
            "waterfall-image",
            image
        );

        display_image.show_size(ui, size);
    }
}
