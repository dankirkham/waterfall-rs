use egui::*;

use crate::configuration::Configuration;

pub struct Settings<'a> {
    config: &'a mut Configuration,
}

impl<'a> Settings<'a> {
    pub fn new(config: &'a mut Configuration) -> Self {
        Self { config }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
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
    }
}
