mod app;
mod configuration;
mod plot_data;
mod recorder;
mod turbo;
mod waterfall_plot;
mod waterfall_processor;
mod waterfall_ticks;

use app::App;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Waterfall",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
