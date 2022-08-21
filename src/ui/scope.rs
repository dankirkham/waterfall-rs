use egui::plot::{Line, Plot, Value, Values};
use egui::*;

use crate::recorder::RecorderData;

pub struct Scope<'a> {
    plot_data: &'a Vec<RecorderData>,
}

fn trigger_position(data: &Vec<RecorderData>, threshold: f32) -> Option<usize> {
    let below = data.iter().position(|d| d < &threshold)?;
    data[below..].iter().position(|d| d > &threshold)
}

impl<'a> Scope<'a> {
    pub fn new(plot_data: &'a Vec<RecorderData>) -> Self {
        Self { plot_data }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let plot_data = match trigger_position(&self.plot_data, 0.0) {
            Some(start_pos) => &self.plot_data[start_pos..],
            None => &self.plot_data,
        };

        let line = Line::new(Values::from_ys_f32(plot_data));
        Plot::new("Scope")
            // .view_aspect(1.732)
            .center_y_axis(true)
            // .include_y(1.0)
            // .include_y(-1.0)
            .include_x(450.0)
            .height(200.0)
            .width(400.0)
            .show(ui, |plot_ui| plot_ui.line(line));
    }
}
