mod app;
mod configuration;
mod plot_data;
mod recorder;
mod turbo;
mod waterfall_plot;
mod waterfall_processor;

use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock, mpsc};

use app::App;
use plot_data::{PlotRow};
use recorder::{Recorder, RecorderData};
use waterfall_processor::WaterfallProcessor;
use configuration::{Configuration, GlobalConfig};

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();
    let config = Arc::new(RwLock::new(Configuration::default()));

    let (tx, rx) = mpsc::channel::<PlotRow>();
    let (sample_tx, sample_rx) = mpsc::channel::<RecorderData>();

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

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Waterfall",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, rx))),
    );
}
