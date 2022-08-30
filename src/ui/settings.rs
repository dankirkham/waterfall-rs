use egui::*;

use crate::configuration::Configuration;
use crate::configuration::DecoderType;
use crate::input::InputSource;

pub struct Settings<'a> {
    config: &'a mut Configuration,
}

impl<'a> Settings<'a> {
    pub fn new(config: &'a mut Configuration) -> Self {
        Self { config }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Source");
            });
            egui::ComboBox::from_label("Source")
                .selected_text(format!("{:?}", self.config.input_source))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.input_source, InputSource::Audio, "Audio");
                    ui.selectable_value(&mut self.config.input_source, InputSource::Synth, "Synth");
                });
            egui::ComboBox::from_label("Sample Rate")
                .selected_text(format!("{:?}", self.config.audio_sample_rate))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.audio_sample_rate, 8000, "8 kHz");
                    ui.selectable_value(&mut self.config.audio_sample_rate, 16000, "16 kHz");
                    ui.selectable_value(&mut self.config.audio_sample_rate, 22050, "22.05 kHz");
                    ui.selectable_value(&mut self.config.audio_sample_rate, 44100, "44.1 kHz");
                    ui.selectable_value(&mut self.config.audio_sample_rate, 48000, "48 kHz");
                    ui.selectable_value(&mut self.config.audio_sample_rate, 96000, "96 kHz");
                });

            ui.separator();
            ui.vertical_centered(|ui| {
                ui.heading("Waterfall");
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

            ui.separator();
            ui.vertical_centered(|ui| {
                ui.heading("Tuner");
            });
            egui::Grid::new("tuner_settings").show(ui, |ui| {
                ui.add(
                    egui::DragValue::new(&mut self.config.tuner.carrier)
                        // .custom_formatter(|n, _| format!("{}", Frequency::Hertz(n)))
                );
                ui.label("Carrier");
                ui.end_row();

                ui.add(
                    egui::DragValue::new(&mut self.config.tuner.lower)
                        // .custom_formatter(|n, _| format!("{}", Frequency::Hertz(n)))
                );
                ui.label("Bandpass Lower");
                ui.end_row();

                ui.add(
                    egui::DragValue::new(&mut self.config.tuner.upper)
                        // .custom_formatter(|n, _| format!("{}", Frequency::Hertz(n)))
                );
                ui.label("Bandpass Upper");
                ui.end_row();
            });
            egui::ComboBox::from_label("Decoder")
                .selected_text(format!("{:?}", self.config.tuner.decoder))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.tuner.decoder, DecoderType::Ft8, "FT8");
                });
        });
    }
}
