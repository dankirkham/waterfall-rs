use indicatif::ParallelProgressIterator;
use plotters::prelude::*;
use rand::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::mpsc;

use waterfall_rs::configuration::{AudioSampleRate, Configuration};
use waterfall_rs::dsp::rx::Rx;
use waterfall_rs::input::{InstantSynth, InstantSynthBuilder, Source};
use waterfall_rs::message::Message;
use waterfall_rs::statistics::Statistics;
use waterfall_rs::types::SampleType;
use waterfall_rs::units::Frequency;

const DELAY_SWATH: usize = 2;
const ITERATIONS: usize = 1;

#[derive(Debug)]
struct Run {
    noise: f32,
    delay: usize,
    pass: bool,
    iterations: usize,
}

impl Default for Run {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            // noise: 0.1 * rng.gen::<f32>(),
            noise: 0.,
            delay: rng.gen_range(0..DELAY_SWATH),
            pass: false,
            iterations: 0,
        }
    }
}

impl Run {
    pub fn synth(&self, builder: InstantSynthBuilder) -> InstantSynth {
        builder
            .with_delay(self.delay)
            .with_noise(self.noise)
            .build()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::default();
    // config.audio_sample_rate = AudioSampleRate::F44100;
    let symbol_rate = Frequency::Hertz(6.25);
    let samples_per_symbol = config.audio_sample_rate.baseband_sample_rate() / symbol_rate;
    println!("Samples per symbol: {}", samples_per_symbol);

    let v: Vec<_> = (0..DELAY_SWATH)
        .into_iter()
        .map(|delay| (0..ITERATIONS).into_iter().map(move |_| delay))
        .flatten()
        .collect();

    let runs: Vec<Run> = v
        .par_iter()
        .progress_count(v.len() as u64)
        .map(|delay| {
            let (sample_tx, mut sample_rx) = mpsc::channel::<Vec<SampleType>>(1024);
            let (message_tx, mut message_rx) = mpsc::channel::<Box<dyn Message>>(5);

            let mut stats = Statistics::default();

            let mut run = Run::default();
            run.delay = *delay;
            let mut rx = Rx::new(&config).with_message_sender(message_tx.clone());
            let mut synth = run.synth(InstantSynthBuilder::new(
                sample_tx.clone(),
                config.audio_sample_rate,
            ));

            'inner: for i in 0..2000 {
                synth.run(&config);

                if let Ok(samples) = sample_rx.try_recv() {
                    rx.run(samples, &config, &mut stats);
                }

                if let Ok(_) = message_rx.try_recv() {
                    run.iterations = i;
                    run.pass = true;
                    break 'inner;
                }
            }

            run
        })
        .collect();

    let mut passes_by_delay: Vec<usize> = vec![0; DELAY_SWATH];
    // dbg!(&runs);
    runs.iter().for_each(|run| {
        if run.pass {
            passes_by_delay[run.delay] += 1;
        }
    });
    let data: Vec<_> = passes_by_delay
        .into_iter()
        .enumerate()
        .map(|(delay, passes)| (delay, passes))
        .collect();

    // dbg!(runs);
    const OUT_FILE_NAME: &'static str = "ft8_sync.gif";
    let root = BitMapBackend::gif(OUT_FILE_NAME, (1920, 1080), 100)?.into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("FT8 Performance", ("sans-serif", 20))
        .set_all_label_area_size(50)
        .margin(4)
        .build_cartesian_2d((0..(DELAY_SWATH - 1)).into_segmented(), 0_usize..ITERATIONS)?;

    chart
        .configure_mesh()
        .x_labels(20)
        .y_labels(10)
        // .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|(delay, passes)| (*delay, *passes))),
    )?;

    root.present()?;

    Ok(())
}
