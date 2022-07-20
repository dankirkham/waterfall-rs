use std::sync::mpsc::Receiver;

use egui::*;
use egui_extras::image::RetainedImage;
use image::{RgbaImage, Rgba, imageops};

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

        let mut image = RgbaImage::new(data_width as u32, PLOT_DEPTH as u32);
        for y in 0..data_height {
            let row = &data[y];
            for x in 0..data_width {
                let sample = row[x];
                let [red, green, blue] = get_color(sample.into());
                let color = Rgba([red, green, blue, 255]);
                let x_min = x as u32;
                let y_min = (y + PLOT_DEPTH - data_height) as u32;

                image.put_pixel(x_min, y_min, color);

            }
        }

        let size = ui.available_size();

        let color_image = ColorImage::from_rgba_unmultiplied(
            [data_width, PLOT_DEPTH],
            &image
        );

        let display_image = RetainedImage::from_color_image(
            "waterfall-image",
            color_image
        );

        display_image.show(ui);
    }
}
