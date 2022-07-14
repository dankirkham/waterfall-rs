use egui::*;
use crate::plot_data::{PlotData, PlotRow, PLOT_DEPTH};
use crate::turbo::get_color;
use std::sync::mpsc::Receiver;

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

        let data_width = data[0].len();
        if data_width == 0 {
            return;
        }

        let size = ui.available_size();
        let size_x = size.x / data_width as f32;
        let size_y = size.y / PLOT_DEPTH as f32;

        let (response, painter) = ui.allocate_painter(size, Sense::hover());
        let rect = response.rect;

        let mut shapes: Vec<Shape> = Vec::with_capacity(data_width * data_height);
        for y in 0..data_height {
            let row = &data[y];
            for x in 0..data_width {
                let sample = row[x];
                let [red, green, blue] = get_color(sample.into());
                let color = Color32::from_rgb(red, green, blue);
                let min = Pos2 {
                    x: rect.left() + x as f32 * size_x,
                    y: rect.top() + (y + PLOT_DEPTH - data_height) as f32 * size_y,
                };

                let max = Pos2 {
                    x: min.x + size_x,
                    y: min.y + size_y,
                };

                let r = Shape::rect_filled(
                    Rect { min, max },
                    Rounding::none(),
                    color
                    );

                shapes.push(r);
            }
        }
        painter.extend(shapes);
    }
}
