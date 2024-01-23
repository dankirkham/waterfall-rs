use egui::*;
use egui_extras::image::RetainedImage;
use tokio::sync::mpsc;
use wasm_timer::Instant;

use crate::configuration::Configuration;
use crate::dsp::Processor;
use crate::input::{Audio, Example, InputSource, Source, Synth};
use crate::message::Message;
use crate::scope::Scope;
use crate::statistics::Statistics;
use crate::types::SampleType;
use crate::ui::{About, Messages, ScopeViewer, Settings, Toolbar, WaterfallPlot, Windows};
use crate::units::Time;

pub struct App {
    image_rx: mpsc::Receiver<ColorImage>,
    image: Option<RetainedImage>,

    config: Configuration,

    processor: Processor,

    scope: Scope,

    source: Box<dyn Source>,
    input_source: InputSource,

    show: Windows,

    stats: Statistics,

    input_devices: Vec<String>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (image_tx, image_rx) = mpsc::channel::<ColorImage>(5);
        let (sample_tx, sample_rx) = mpsc::channel::<Vec<SampleType>>(1024);
        let (plot_tx, plot_rx) = mpsc::channel::<Vec<SampleType>>(5);
        let (message_tx, _message_rx) = mpsc::channel::<Box<dyn Message>>(5);

        let config = Configuration::default();

        let processor = Processor::new(sample_rx, image_tx, plot_tx, message_tx, &config);

        let scope = Scope::new(plot_rx);

        let input_source = config.input_source;
        let source = Self::create_source(&config, sample_tx);

        let input_devices = Audio::get_devices();

        Self {
            image_rx,
            image: None,
            config,

            processor,

            scope,

            source,
            input_source,

            show: Windows::default(),

            stats: Statistics::default(),

            input_devices,
        }
    }

    fn create_source(config: &Configuration, tx: mpsc::Sender<Vec<SampleType>>) -> Box<dyn Source> {
        match config.input_source {
            InputSource::Synth => Box::new(Synth::new(tx, config)),
            InputSource::Audio => Box::new(Audio::new(tx, config)),
            InputSource::Example => Box::new(Example::new(tx, config)),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();

        if self.input_source != self.config.input_source {
            self.input_source = self.config.input_source;
            let tx = self.source.get_tx();
            self.source = Self::create_source(&self.config, tx);
        }

        self.source.run(&self.config);
        self.processor.run(&self.config, &mut self.stats);

        self.scope.run(&mut self.config);

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
            let ri = RetainedImage::from_color_image("waterfall-image", im.to_owned())
                .with_texture_filter(TextureFilter::Nearest);
            self.image = Some(ri);
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let mut toolbar = Toolbar::new(&mut self.show);
            toolbar.ui(ui);
        });

        egui::Window::new("🔧 Settings")
            .open(&mut self.show.settings)
            .show(ctx, |ui| {
                let mut settings = Settings::new(&mut self.config, &self.input_devices);
                settings.ui(ui);
            });

        egui::Window::new("🗠 Oscilloscope")
            .open(&mut self.show.scope)
            .show(ctx, |ui| {
                let mut scope = ScopeViewer::new(&mut self.config, self.scope.get_plot_data());
                scope.ui(ui);
            });

        egui::Window::new("❔ About")
            .open(&mut self.show.about)
            .show(ctx, |ui| {
                let mut about = About::default();
                about.ui(ui);
            });

        egui::Window::new("📻 Messages")
            .open(&mut self.show.messages)
            .show(ctx, |ui| {
                let mut messages = Messages::default();
                messages.ui(ui);
            });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(avg) = self.stats.rx.avg() {
                    let t: Time = avg.into();
                    ui.label(format!("RX: {}", t));
                }
                if let Some(avg) = self.stats.waterfall.avg() {
                    let t: Time = avg.into();
                    ui.label(format!("Waterfall: {}", t));
                }
                if let Some(avg) = self.stats.render.avg() {
                    let t: Time = avg.into();
                    ui.label(format!("Render: {}", t));
                }
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    ui.label("FT-8 Sync ⭕");
                });
            });
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

        // let fft_depth = self.config.fft_depth;
        // let audio_sample_rate = self.config.audio_sample_rate;
        // ctx.request_repaint_after(std::time::Duration::from_millis(
        //     (fft_depth as f32 / audio_sample_rate as f32 * 1000.0) as u64,
        // ));

        ctx.request_repaint_after(std::time::Duration::from_millis(1000 / 60));

        let elapsed = now.elapsed();
        self.stats.render.push(elapsed);
    }
}
