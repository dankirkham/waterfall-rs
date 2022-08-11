use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::configuration::Configuration;
use crate::filter::filter::Filter;
use crate::filter::high_pass_filter::HighPassFilter;
use crate::units::Frequency;

pub type RecorderData = f32;

pub struct Recorder {
    sender: Sender<Vec<RecorderData>>,
    // config: Arc<RwLock<Configuration>>,
    sample_rate: i32,
}

impl Recorder {
    pub fn new(sender: Sender<Vec<RecorderData>>, config: Arc<RwLock<Configuration>>) -> Self {
        let sample_rate = config.read().unwrap().audio_sample_rate as i32;

        Self {
            sender,
            // config,
            sample_rate,
        }
    }

    // fn read_callback(&self, stream: &mut soundio::InStreamReader) {
    //     let mut frames_left = stream.frame_count_max();

    //     loop {
    //         if let Err(e) = stream.begin_read(frames_left) {
    //             println!("Error reading from stream: {}", e);
    //             return;
    //         }
    //         for f in 0..stream.frame_count() {
    //             for c in 0..stream.channel_count() {
    //                 let sample = stream.sample::<RecorderData>(c, f);
    //                 self.sender.send(sample).unwrap();
    //             }
    //         }

    //         frames_left -= stream.frame_count();
    //         if frames_left == 0 {
    //             break;
    //         }

    //         stream.end_read();
    //     }
    // }

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

        let err_fn = move |err| {
            // react to errors here.
        };

        let sender = self.sender.clone();
        let mut filter = HighPassFilter::from_frequency(
            Frequency::Hertz(300.0),
            Frequency::Hertz(self.sample_rate as f32),
        );

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    let mut samples = data.into_iter().step_by(2).map(|x| *x);

                    let mut filtered: Vec<f32> =
                        samples.map(|sample| filter.next(sample)).collect();

                    sender.send(filtered);
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
