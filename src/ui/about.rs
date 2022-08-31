use std::env;
use std::fmt;

use egui::*;

pub struct About;

impl About {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("waterfall-rs");
        let version: &'static str = option_env!("WATERFALL_VERSION").unwrap_or("development");
        ui.label(format!("Build: {version}"));
        ui.add_space(10.0);
        ui.label("A software-defined radio (SDR) tool written in Rust.");
        ui.add_space(10.0);
        ui.label("by Dan Kirkham");
        ui.add_space(10.0);
        ui.hyperlink_to(
            "source code on github",
            "https://github.com/dankirkham/waterfall-rs",
        );
    }
}
