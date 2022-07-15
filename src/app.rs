use egui::*;
use std::sync::mpsc::Receiver;

use crate::plot_data::{PlotData, PlotRow, new_plot_data};
use crate::waterfall_plot::WaterfallPlot;

pub struct App {
    plot_row_rx: Receiver<PlotRow>,
    plot_data: PlotData,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>, plot_row_rx: Receiver<PlotRow>) -> Self {
        Self {
            plot_row_rx,
            plot_data: new_plot_data(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 frame.quit();
        //             }
        //         });
        //     });
        // });

        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                let mut waterfall = WaterfallPlot::new(
                    &mut self.plot_row_rx,
                    &mut self.plot_data
                );
                waterfall.ui(ui);
            });

        // egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //         egui::warn_if_debug_build(ui);
        //     });
        // });

        ctx.request_repaint(); // Continuous mode
    }
}
