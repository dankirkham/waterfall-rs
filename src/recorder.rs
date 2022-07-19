use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};

use soundio;

use crate::configuration::Configuration;

pub type RecorderData = f32;

pub struct Recorder {
    sender: Sender<RecorderData>,
    config: Arc<RwLock<Configuration>>,
    sample_rate: i32,
}

impl Recorder {
    pub fn new(sender: Sender<RecorderData>, config: Arc<RwLock<Configuration>>) -> Self {
        let sample_rate = config.read().unwrap().audio_sample_rate as i32;

        Self {
            sender,
            config,
            sample_rate,
        }
    }

    fn read_callback(&self, stream: &mut soundio::InStreamReader) {
        let mut frames_left = stream.frame_count_max();

        loop {
            if let Err(e) = stream.begin_read(frames_left) {
                println!("Error reading from stream: {}", e);
                return;
            }
            for f in 0..stream.frame_count() {
                for c in 0..stream.channel_count() {
                    let sample = stream.sample::<RecorderData>(c, f);
                    self.sender.send(sample).unwrap();
                }
            }

            frames_left -= stream.frame_count();
            if frames_left <= 0 {
                break;
            }

            stream.end_read();
        }
    }


    pub fn start(&self) {
        let mut ctx = soundio::Context::new();
        ctx.set_app_name("Waterfall");

        ctx.connect().expect("Failed to connect to sound backend");
        println!("Current backend: {:?}", ctx.current_backend());

        ctx.flush_events();

        let dev = ctx.default_input_device().expect("No input device");
        println!(
            "Default input device: {} ({})",
            dev.name(),
            if dev.is_raw() { "raw" } else { "not raw" }
        );

        let mut input_stream = dev.open_instream(
            self.sample_rate,
            soundio::Format::S16LE,
            soundio::ChannelLayout::get_builtin(soundio::ChannelLayoutId::Mono),
            0.1,
            |x| self.read_callback(x),
            None::<fn()>,
            None::<fn(soundio::Error)>,
        ).unwrap();

        input_stream.start();

        loop {
            ctx.wait_events();
        }
    }
}
