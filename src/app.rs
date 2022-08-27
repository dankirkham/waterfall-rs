use std::sync::mpsc;

use egui::*;
use egui_extras::image::RetainedImage;

use crate::configuration::Configuration;
use crate::dsp::Processor;
use crate::input::Source;
use crate::types::SampleType;
use crate::ui::{Scope, Settings, WaterfallPlot};

pub struct App {
    image_rx: mpsc::Receiver<ColorImage>,
    image: Option<RetainedImage>,

    config: Configuration,

    plot_rx: mpsc::Receiver<Vec<SampleType>>,
    plot_data: Vec<SampleType>,

    processor: Processor,
    source: Source,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (image_tx, image_rx) = mpsc::channel::<ColorImage>();
        let (sample_tx, sample_rx) = mpsc::channel::<Vec<SampleType>>();
        let (plot_tx, plot_rx) = mpsc::channel::<Vec<SampleType>>();

        let config = Configuration::default();

        let source = Source::new(sample_tx, &config);
        let processor = Processor::new(sample_rx, image_tx, plot_tx, &config);

        Self {
            image_rx,
            image: None,
            config,

            plot_rx,
            plot_data: Vec::new(),

            processor,
            source,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

        egui::Window::new("Scope").show(ctx, |ui| {
            let mut scope = Scope::new(&self.plot_data);
            scope.ui(ui);
        });

        egui::Window::new("Settings").show(ctx, |ui| {
            let mut settings = Settings::new(&mut self.config);
            settings.ui(ui);
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
