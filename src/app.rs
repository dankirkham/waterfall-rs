use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock, mpsc};
use std::thread;
use std::time::Duration;

use egui::*;

use crate::plot_data::{PlotData, PlotRow, new_plot_data};
use crate::waterfall_plot::WaterfallPlot;
use crate::recorder::{Recorder, RecorderData};
use crate::waterfall_processor::WaterfallProcessor;
use crate::configuration::{Configuration, GlobalConfig};


pub struct App {
    plot_row_rx: Receiver<PlotRow>,
    plot_data: PlotData,
    config: GlobalConfig,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = mpsc::channel::<PlotRow>();
        let (sample_tx, sample_rx) = mpsc::channel::<RecorderData>();

        let config = Arc::new(RwLock::new(Configuration::default()));
        let r_config = config.clone();
        thread::spawn(move || {
            let mut recorder = Recorder::new(sample_tx, r_config);
            recorder.start();
        });

        let p_config = config.clone();
        thread::spawn(move || {
            let mut processor = WaterfallProcessor::new(sample_rx, tx, p_config);
            processor.start();
        });

        Self {
            plot_row_rx: rx,
            plot_data: new_plot_data(),
            config,
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

        // egui::SidePanel::left("waterfall_settings").show(ctx, |ui| {
        //    ui.label("Waterfall Settings");
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

        let fft_depth = self.config.read().unwrap().fft_depth;
        let audio_sample_rate = self.config.read().unwrap().audio_sample_rate;

        ctx.request_repaint_after(std::time::Duration::from_millis(
            (fft_depth / audio_sample_rate) as u64
        ));
    }
}
