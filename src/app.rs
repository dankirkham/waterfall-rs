use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use egui::*;

use crate::configuration::Configuration;
use crate::plot_data::{new_plot_data, PlotData, PlotRow};
use crate::recorder::{Recorder, RecorderData};
use crate::waterfall_plot::WaterfallPlot;
use crate::waterfall_processor::WaterfallProcessor;

pub struct App {
    plot_row_rx: mpsc::Receiver<PlotRow>,
    plot_data: PlotData,

    safe_config: Arc<RwLock<Configuration>>,
    config: Configuration,
    edit_config: Configuration,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = mpsc::channel::<PlotRow>();
        let (sample_tx, sample_rx) = mpsc::channel::<RecorderData>();

        let config = Configuration::default();
        let edit_config = config;
        let safe_config = Arc::new(RwLock::new(config));

        let r_config = safe_config.clone();
        thread::spawn(move || {
            let recorder = Recorder::new(sample_tx, r_config);
            recorder.start();
        });

        let p_config = safe_config.clone();
        thread::spawn(move || {
            let mut processor = WaterfallProcessor::new(sample_rx, tx, p_config);
            processor.start();
        });

        Self {
            plot_row_rx: rx,
            plot_data: new_plot_data(),
            config,
            edit_config,
            safe_config,
        }
    }

    fn update_config(&mut self) {
        self.config = self.edit_config;
        let mut sf = self.safe_config.write().unwrap();
        *sf = self.config;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 frame.quit();
        //             }
        //         });
        //     });
        // });

        egui::SidePanel::right("waterfall_settings")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Waterfall Settings");
                egui::ComboBox::from_label("Sample Rate")
                    .selected_text(format!("{:?}", self.edit_config.audio_sample_rate))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.edit_config.audio_sample_rate,
                            44100,
                            "44.1 kHz",
                        );
                        ui.selectable_value(
                            &mut self.edit_config.audio_sample_rate,
                            48000,
                            "48 kHz",
                        );
                        ui.selectable_value(
                            &mut self.edit_config.audio_sample_rate,
                            96000,
                            "96 kHz",
                        );
                    });
                egui::ComboBox::from_label("FFT Depth")
                    .selected_text(format!("{:?}", self.edit_config.fft_depth))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.edit_config.fft_depth, 1024, "1024");
                        ui.selectable_value(&mut self.edit_config.fft_depth, 2048, "2048");
                        ui.selectable_value(&mut self.edit_config.fft_depth, 4096, "4096");
                        ui.selectable_value(&mut self.edit_config.fft_depth, 8192, "8192");
                        ui.selectable_value(&mut self.edit_config.fft_depth, 16384, "16384");
                    });
                egui::ComboBox::from_label("Trim (Hz)")
                    .selected_text(format!("{:?}", self.edit_config.trim_hz))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.edit_config.trim_hz, 4000, "4000 Hz");
                        ui.selectable_value(&mut self.edit_config.trim_hz, 8000, "8000 Hz");
                        ui.selectable_value(&mut self.edit_config.trim_hz, 22050, "22050 Hz");
                        ui.selectable_value(&mut self.edit_config.trim_hz, 24000, "24000 Hz");
                        ui.selectable_value(&mut self.edit_config.trim_hz, 48000, "48000 Hz");
                    });
                ui.add(
                    egui::Slider::new(
                        &mut self.edit_config.min_db,
                        -50.0..=self.edit_config.max_db,
                    )
                    .text("Min dB"),
                );
                ui.add(
                    egui::Slider::new(
                        &mut self.edit_config.max_db,
                        self.edit_config.min_db..=100.0,
                    )
                    .text("Max dB"),
                );
                if ui.add(egui::Button::new("Apply")).clicked() {
                    self.update_config();
                }
            });

        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                let mut waterfall = WaterfallPlot::new(&mut self.plot_row_rx, &mut self.plot_data);
                waterfall.ui(ui);
            });

        // egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //         egui::warn_if_debug_build(ui);
        //     });
        // });

        let fft_depth = self.config.fft_depth;
        let audio_sample_rate = self.config.audio_sample_rate;
        ctx.request_repaint_after(std::time::Duration::from_millis(
            (fft_depth as f32 / audio_sample_rate as f32 * 1000.0) as u64,
        ));
    }
}
