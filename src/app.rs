use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use egui::*;
use egui_extras::image::RetainedImage;

use crate::configuration::Configuration;
use crate::dsp::WaterfallProcessor;
use crate::recorder::{Recorder, RecorderData};
use crate::ui::{Scope, WaterfallPlot};

pub struct App {
    image_rx: mpsc::Receiver<ColorImage>,
    image: Option<RetainedImage>,

    safe_config: Arc<RwLock<Configuration>>,
    config: Configuration,

    plot_rx: mpsc::Receiver<Vec<RecorderData>>,
    plot_data: Vec<RecorderData>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (image_tx, image_rx) = mpsc::channel::<ColorImage>();
        let (sample_tx, sample_rx) = mpsc::channel::<Vec<RecorderData>>();
        let (plot_tx, plot_rx) = mpsc::channel::<Vec<RecorderData>>();

        let config = Configuration::default();
        let safe_config = Arc::new(RwLock::new(config));

        let r_config = safe_config.clone();
        thread::spawn(move || {
            let recorder = Recorder::new(sample_tx, r_config);
            recorder.start();
        });

        let p_config = safe_config.clone();
        thread::spawn(move || {
            let mut processor = WaterfallProcessor::new(sample_rx, image_tx, plot_tx, p_config);
            processor.start();
        });

        Self {
            image_rx,
            image: None,
            config,
            safe_config,

            plot_rx,
            plot_data: Vec::new(),
        }
    }

    fn update_config(&mut self) {
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

        while let Ok(im) = self.image_rx.try_recv() {
            let ri = RetainedImage::from_color_image("waterfall-image", im.to_owned());
            self.image = Some(ri);
        }

        while let Ok(plot_data) = self.plot_rx.try_recv() {
            self.plot_data = plot_data;
        }

        egui::Window::new("Scope").show(ctx, |ui| {
            let mut scope = Scope::new(&self.plot_data);
            scope.ui(ui);
        });

        egui::SidePanel::right("waterfall_settings")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Waterfall Settings");
                egui::ComboBox::from_label("Sample Rate")
                    .selected_text(format!("{:?}", self.config.audio_sample_rate))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.audio_sample_rate, 44100, "44.1 kHz");
                        ui.selectable_value(&mut self.config.audio_sample_rate, 48000, "48 kHz");
                        ui.selectable_value(&mut self.config.audio_sample_rate, 96000, "96 kHz");
                    });
                egui::ComboBox::from_label("FFT Depth")
                    .selected_text(format!("{:?}", self.config.fft_depth))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.fft_depth, 1024, "1024");
                        ui.selectable_value(&mut self.config.fft_depth, 2048, "2048");
                        ui.selectable_value(&mut self.config.fft_depth, 4096, "4096");
                        ui.selectable_value(&mut self.config.fft_depth, 8192, "8192");
                        ui.selectable_value(&mut self.config.fft_depth, 16384, "16384");
                    });
                egui::ComboBox::from_label("Trim (Hz)")
                    .selected_text(format!("{:?}", self.config.trim_hz))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.trim_hz, 4000, "4000 Hz");
                        ui.selectable_value(&mut self.config.trim_hz, 8000, "8000 Hz");
                        ui.selectable_value(&mut self.config.trim_hz, 22050, "22050 Hz");
                        ui.selectable_value(&mut self.config.trim_hz, 24000, "24000 Hz");
                        ui.selectable_value(&mut self.config.trim_hz, 48000, "48000 Hz");
                    });
                ui.add(
                    egui::Slider::new(&mut self.config.min_db, -50.0..=self.config.max_db)
                        .text("Min dB"),
                );
                ui.add(
                    egui::Slider::new(&mut self.config.max_db, self.config.min_db..=100.0)
                        .text("Max dB"),
                );
                ui.add(egui::Slider::new(&mut self.config.zoom, 1.0..=5.0).text("Zoom"));
                ui.add(egui::Slider::new(&mut self.config.scroll, 0.0..=1.0).text("Scroll"));

                ui.label("Tuner Settings");
                ui.label(format!("Bandpass Low: {}", self.config.tuner.lower));
                ui.label(format!("Bandpass High: {}", self.config.tuner.upper));
                ui.label(format!("Carrier: {}", self.config.tuner.carrier));

                if ui.add(egui::Button::new("Apply")).clicked() {
                    self.update_config();
                }
            });

        egui::CentralPanel::default()
            .frame(Frame::none().fill(ctx.style().visuals.faint_bg_color))
            .show(ctx, |ui| {
                let mut waterfall = WaterfallPlot::new(&self.image, &mut self.config);
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
