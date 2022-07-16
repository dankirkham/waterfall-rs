use std::ops::Deref;
use std::sync::mpsc::Sender;

use soundio;

use crate::configuration::GlobalConfig;

pub type RecorderData = f32;

pub struct Recorder {
    sender: Sender<RecorderData>,
    config: GlobalConfig,
    sample_rate: i32,
}

impl Recorder {
    pub fn new(sender: Sender<RecorderData>, config: GlobalConfig) -> Self {
        let sample_rate = config.read().unwrap().audio_sample_rate as i32;
        Self {
            sender,
            sample_rate,
            config,
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

        // let backends = ctx.available_backends();
        // println!();
        // println!("Backends:");
        // for backend in backends {
        //     println!("{}", backend);
        // }
        // println!();

        ctx.connect().expect("Failed to connect to sound backend");
        println!("Current backend: {:?}", ctx.current_backend());

        ctx.flush_events();

        // let devices = ctx.input_devices().unwrap();
        // println!();
        // println!("Devices:");
        // for device in devices {
        //     println!("{} {}", device.id(), device.name());
        // }
        // println!();

        let dev = ctx.default_input_device().expect("No input device");
        println!(
            "Default input device: {} ({})",
            dev.name(),
            if dev.is_raw() { "raw" } else { "not raw" }
        );

        // fn write_callback(stream: &mut soundio::OutStreamWriter) {
        //     let mut rng = rand::thread_rng();
        //     println!("frame_count_min: {}", stream.frame_count_min());
        //     println!("frame_count_max: {}", stream.frame_count_max());

        //     let frame_count_max = stream.frame_count_max();
        //     stream.begin_write(frame_count_max).unwrap();
        //     for c in 0..stream.channel_count() {
        //         for f in 0..stream.frame_count() {
        //             // stream.set_sample::<f32>(c, f, 0.0 as f32);
        //             stream.set_sample::<f32>(c, f, rng.gen());
        //         }
        //     }
        // }

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
            let sample_rate = self.config.read().unwrap().audio_sample_rate as i32;
            if sample_rate as i32 != self.sample_rate {
                println!("The sample rate changed and we need to reconfigure the audio device.");
            }
            // ctx.wait_events();
        }
    }
}
