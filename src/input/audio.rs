use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;

use crate::configuration::Configuration;
use crate::filter::Filter;
use crate::filter::HighPassFilter;
use crate::input::Source;
use crate::types::SampleType;
use crate::units::Frequency;

pub struct Audio {
    sender: Sender<Vec<SampleType>>,
    stream: Stream,

    sample_rate: usize,
    device_name: String,
}

impl Audio {
    pub fn get_devices() -> Vec<String> {
        let host = cpal::default_host();
        host.input_devices()
            .unwrap()
            .into_iter()
            .map(|d| d.name().unwrap())
            .collect()
    }

    pub fn new(sender: Sender<Vec<SampleType>>, config: &Configuration) -> Self {
        let sample_rate = config.audio_sample_rate;
        let sample_rate_f = Frequency::Hertz(sample_rate as f32);

        let host = cpal::default_host();

        let device_name = config.input_device.to_string();
        let device = if device_name == "Default" {
            host.default_input_device().expect("No input device")
        } else {
            host.input_devices()
                .unwrap()
                .into_iter()
                .find(|d| d.name().unwrap() == device_name)
                .unwrap()
        };

        println!("Using device {}", device.name().unwrap());

        let mut supported_configs_range = device
            .supported_input_configs()
            .expect("error while querying configs");

        let config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        let err_fn = move |_err| {
            // react to errors here.
        };

        let mut filter = HighPassFilter::from_frequency(Frequency::Hertz(300.0), sample_rate_f);

        let stream_sender = sender.clone();
        let stream = match config.channels() {
            1 => match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &_| {
                        let samples = data.iter().copied();

                        let filtered: Vec<f32> = samples.map(|sample| filter.next(sample)).collect();
                        // let filtered: Vec<f32> = samples.map(|sample| ft8.next()).collect();

                        if let Err(err) = stream_sender.try_send(filtered) {
                            match err {
                                TrySendError::Full(_) => println!("Waterfall processor falling behind"),
                                TrySendError::Closed(_) => (),
                            }
                        }
                    },
                    err_fn,
                ),
                cpal::SampleFormat::I16 => device.build_input_stream(
                    &config.into(),
                    move |data: &[i16], _: &_| {
                        let samples = data
                            .iter()
                            .map(|&v| v as f32)
                            .map(|v| v / 32768.0);

                        let filtered: Vec<f32> = samples.map(|sample| filter.next(sample)).collect();

                        if let Err(err) = stream_sender.try_send(filtered) {
                            match err {
                                TrySendError::Full(_) => println!("Waterfall processor falling behind"),
                                TrySendError::Closed(_) => (),
                            }
                        }
                    },
                    err_fn,
                ),
                cpal::SampleFormat::U16 => device.build_input_stream(
                    &config.into(),
                    move |data: &[u16], _: &_| {
                        let samples = data
                            .iter()
                            .map(|&v| v as f32)
                            .map(|v| v / 32768.0)
                            .map(|v| v - 1.0);

                        let filtered: Vec<f32> = samples.map(|sample| filter.next(sample)).collect();

                        if let Err(err) = stream_sender.try_send(filtered) {
                            match err {
                                TrySendError::Full(_) => println!("Waterfall processor falling behind"),
                                TrySendError::Closed(_) => (),
                            }
                        }
                    },
                    err_fn,
                ),
            },
            2 => match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &_| {
                        let samples = data.iter().step_by(2).copied();

                        let filtered: Vec<f32> = samples.map(|sample| filter.next(sample)).collect();
                        // let filtered: Vec<f32> = samples.map(|sample| ft8.next()).collect();

                        if let Err(err) = stream_sender.try_send(filtered) {
                            match err {
                                TrySendError::Full(_) => println!("Waterfall processor falling behind"),
                                TrySendError::Closed(_) => (),
                            }
                        }
                    },
                    err_fn,
                ),
                cpal::SampleFormat::I16 => device.build_input_stream(
                    &config.into(),
                    move |data: &[i16], _: &_| {
                        let samples = data
                            .iter()
                            .step_by(2)
                            .map(|&v| v as f32)
                            .map(|v| v / 32768.0);

                        let filtered: Vec<f32> = samples.map(|sample| filter.next(sample)).collect();

                        if let Err(err) = stream_sender.try_send(filtered) {
                            match err {
                                TrySendError::Full(_) => println!("Waterfall processor falling behind"),
                                TrySendError::Closed(_) => (),
                            }
                        }
                    },
                    err_fn,
                ),
                cpal::SampleFormat::U16 => device.build_input_stream(
                    &config.into(),
                    move |data: &[u16], _: &_| {
                        let samples = data
                            .iter()
                            .step_by(2)
                            .map(|&v| v as f32)
                            .map(|v| v / 32768.0)
                            .map(|v| v - 1.0);

                        let filtered: Vec<f32> = samples.map(|sample| filter.next(sample)).collect();

                        if let Err(err) = stream_sender.try_send(filtered) {
                            match err {
                                TrySendError::Full(_) => println!("Waterfall processor falling behind"),
                                TrySendError::Closed(_) => (),
                            }
                        }
                    },
                    err_fn,
                ),
            },
            _ => panic!("Only supports 1 or 2 channels"),
        }
        .expect("Unable to build stream");

        stream.play().expect("Unable to play stream");

        println!("Audio initialized");

        Self {
            sender,
            // config,
            sample_rate,
            stream,
            device_name,
        }
    }
}

impl Source for Audio {
    fn run(&mut self, config: &Configuration) {
        if config.audio_sample_rate != self.sample_rate || config.input_device != self.device_name {
            *self = Audio::new(self.sender.clone(), config);
        }
    }

    fn get_tx(&self) -> Sender<Vec<SampleType>> {
        self.sender.clone()
    }
}
