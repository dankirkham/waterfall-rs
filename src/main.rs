mod app;
mod plot_data;
mod recorder;
mod turbo;
mod waterfall_plot;
mod waterfall_processor;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use app::App;
use plot_data::{PlotRow};
use recorder::{Recorder, RecorderData};
use waterfall_processor::WaterfallProcessor;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();

    let (tx, rx) = mpsc::channel::<PlotRow>();
    let (sample_tx, sample_rx) = mpsc::channel::<RecorderData>();

    let recorder = Recorder::new(sample_tx);
    recorder.start();

    thread::spawn(move || {
        let processor = WaterfallProcessor::new(sample_rx, tx);
        processor.start();
    });

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Waterfall",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, rx))),
    );
}
