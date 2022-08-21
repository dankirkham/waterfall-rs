use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::configuration::Configuration;
use crate::filter::Filter;
use crate::filter::HighPassFilter;
use crate::synth::{Ft8, Samples};
use crate::units::Frequency;

pub type RecorderData = f32;

pub struct Recorder {
    sender: Sender<Vec<RecorderData>>,
    // config: Arc<RwLock<Configuration>>,
    sample_rate: Frequency,
}

impl Recorder {
    pub fn new(sender: Sender<Vec<RecorderData>>, config: Arc<RwLock<Configuration>>) -> Self {
        let sample_rate = Frequency::Hertz(config.read().unwrap().audio_sample_rate as f32);

        Self {
            sender,
            // config,
            sample_rate,
        }
    }

    pub fn start(&self) {
        let host = cpal::default_host();

        host.input_devices()
            .unwrap()
            .into_iter()
            .for_each(|d| println!("{}", d.name().unwrap()));

        let device = host.default_input_device().expect("No input device");
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

        let sender = self.sender.clone();
        let mut filter = HighPassFilter::from_frequency(Frequency::Hertz(300.0), self.sample_rate);

        let mut ft8 = Ft8::new(self.sample_rate, Frequency::Hertz(100.0));

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    let samples = data.iter().step_by(2).copied();

                    let filtered: Vec<f32> = samples.map(|sample| filter.next(sample)).collect();
                    // let filtered: Vec<f32> = samples.map(|sample| ft8.next()).collect();

                    sender.send(filtered).unwrap();
                },
                err_fn,
            ),
            _ => panic!("Sample format not supported"),
        }
        .expect("Unable to build stream");

        stream.play().expect("Unable to play stream");

        println!("Audio initialized");
        loop {
            sleep(Duration::from_millis(500));
        }
    }
}
