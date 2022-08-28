use std::sync::mpsc;

use egui::*;
use egui_extras::image::RetainedImage;

use crate::configuration::Configuration;
use crate::dsp::Processor;
use crate::input::{Audio, InputSource, Source, Synth};
use crate::types::SampleType;
use crate::ui::{About, Messages, Scope, Settings, Toolbar, WaterfallPlot, Windows};

pub struct App {
    image_rx: mpsc::Receiver<ColorImage>,
    image: Option<RetainedImage>,

    config: Configuration,

    plot_rx: mpsc::Receiver<Vec<SampleType>>,
    plot_data: Vec<SampleType>,

    processor: Processor,

    source: Box<dyn Source>,
    input_source: InputSource,

    show: Windows,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (image_tx, image_rx) = mpsc::channel::<ColorImage>();
        let (sample_tx, sample_rx) = mpsc::channel::<Vec<SampleType>>();
        let (plot_tx, plot_rx) = mpsc::channel::<Vec<SampleType>>();

        let config = Configuration::default();

        let processor = Processor::new(sample_rx, image_tx, plot_tx, &config);

        let input_source = config.input_source;
        let source = Self::create_source(&config, sample_tx);

        Self {
            image_rx,
            image: None,
            config,

            plot_rx,
            plot_data: Vec::new(),

            processor,

            source,
            input_source,

            show: Windows::default(),
        }
    }

    fn create_source(config: &Configuration, tx: mpsc::Sender<Vec<SampleType>>) -> Box<dyn Source> {
        match config.input_source {
            InputSource::Synth => Box::new(Synth::new(tx, &config)),
            InputSource::Audio => Box::new(Audio::new(tx, &config)),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.input_source != self.config.input_source {
            self.input_source = self.config.input_source;
            let tx = self.source.get_tx();
            self.source = Self::create_source(&self.config, tx);
        }

        self.source.run(&self.config);
        self.processor.run(&self.config);

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

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let mut toolbar = Toolbar::new(&mut self.show);
            toolbar.ui(ui);
        });

        egui::Window::new("🔧 Settings")
            .open(&mut self.show.settings)
            .show(ctx, |ui| {
                let mut settings = Settings::new(&mut self.config);
                settings.ui(ui);
            });

        egui::Window::new("🗠 Oscilloscope")
            .open(&mut self.show.scope)
            .show(ctx, |ui| {
                let mut scope = Scope::new(&self.plot_data);
                scope.ui(ui);
            });

        egui::Window::new("❔ About")
            .open(&mut self.show.about)
            .show(ctx, |ui| {
                let mut about = About::new();
                about.ui(ui);
            });

        egui::Window::new("📻 Messages")
            .open(&mut self.show.messages)
            .show(ctx, |ui| {
                let mut messages = Messages::new();
                messages.ui(ui);
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
