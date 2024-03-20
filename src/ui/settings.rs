use egui::*;

use crate::configuration::DecoderType;
use crate::configuration::{AudioSampleRate, Configuration};
use crate::input::InputSource;
use crate::ui::bump::Bump;

pub struct Settings<'a> {
    config: &'a mut Configuration,
    input_devices: &'a Vec<String>,
}

impl<'a> Settings<'a> {
    pub fn new(config: &'a mut Configuration, input_devices: &'a Vec<String>) -> Self {
        Self {
            config,
            input_devices,
        }
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
                    ui.selectable_value(
                        &mut self.config.input_source,
                        InputSource::Example,
                        "Example",
                    );
                });

            if self.config.input_source == InputSource::Audio {
                egui::ComboBox::from_label("Device")
                    .selected_text(format!("{:?}", self.config.input_device))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.config.input_device,
                            "Default".to_owned(),
                            "Default",
                        );
                        self.input_devices.iter().for_each(|name| {
                            ui.selectable_value(
                                &mut self.config.input_device,
                                name.to_string(),
                                name,
                            );
                        });
                    });
            }

            egui::ComboBox::from_label("Sample Rate")
                .selected_text(format!("{}", self.config.audio_sample_rate))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.config.audio_sample_rate,
                        AudioSampleRate::F8000,
                        AudioSampleRate::F8000.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.config.audio_sample_rate,
                        AudioSampleRate::F16000,
                        AudioSampleRate::F16000.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.config.audio_sample_rate,
                        AudioSampleRate::F22050,
                        AudioSampleRate::F22050.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.config.audio_sample_rate,
                        AudioSampleRate::F44100,
                        AudioSampleRate::F44100.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.config.audio_sample_rate,
                        AudioSampleRate::F48000,
                        AudioSampleRate::F48000.to_string(),
                    );
                    ui.selectable_value(
                        &mut self.config.audio_sample_rate,
                        AudioSampleRate::F96000,
                        AudioSampleRate::F96000.to_string(),
                    );
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
            Bump::new(&mut self.config.tuner.carrier, "Carrier".to_string()).ui(ui);
            Bump::new(&mut self.config.tuner.upper, "Bandpass Upper".to_string()).ui(ui);
            Bump::new(&mut self.config.tuner.lower, "Bandpass Lower".to_string()).ui(ui);

            egui::ComboBox::from_label("Decoder")
                .selected_text(format!("{:?}", self.config.tuner.decoder))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.tuner.decoder, DecoderType::Rtty, "RTTY");
                });
        });
    }
}
