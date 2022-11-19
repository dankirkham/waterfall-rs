use indicatif::ParallelProgressIterator;
use plotters::prelude::*;
use rand::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::mpsc;

use waterfall_rs::configuration::Configuration;
use waterfall_rs::dsp::rx::Rx;
use waterfall_rs::input::{InstantSynth, InstantSynthBuilder, Source};
use waterfall_rs::message::Message;
use waterfall_rs::statistics::Statistics;
use waterfall_rs::types::SampleType;

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
            noise: 0.01 * rng.gen::<f32>(),
            delay: rng.gen_range(0..4096),
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
    let config = Configuration::default();

    let v: Vec<_> = (0..1000).collect();
    let runs: Vec<Run> = v
        .par_iter()
        .progress_count(v.len() as u64)
        .map(|_| {
            let (sample_tx, mut sample_rx) = mpsc::channel::<Vec<SampleType>>(1024);
            let (message_tx, mut message_rx) = mpsc::channel::<Box<dyn Message>>(5);

            let mut stats = Statistics::default();

            let mut run = Run::default();
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

    // dbg!(runs);
    const OUT_FILE_NAME: &'static str = "plot.gif";
    let root = BitMapBackend::gif(OUT_FILE_NAME, (1920, 1080), 100)?.into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("FT8 Performance", ("sans-serif", 20))
        .set_all_label_area_size(50)
        .margin(4)
        .build_cartesian_2d(0.0..0.01, 0.0..4096.0)?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(8)
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    chart
        .draw_series(
            runs.iter()
                .filter(|run| run.pass)
                .map(|run| Circle::new((run.noise as f64, run.delay as f64), 4, GREEN.filled())),
        )?
        .label("Pass")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(
            runs.iter()
                .filter(|run| !run.pass)
                .map(|run| Circle::new((run.noise as f64, run.delay as f64), 4, RED.filled())),
        )?
        .label("Pass")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    root.present()?;

    Ok(())
}
