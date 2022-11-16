use tokio::sync::mpsc;

use waterfall_rs::configuration::Configuration;
use waterfall_rs::dsp::rx::Rx;
use waterfall_rs::input::{InstantSynth, Source};
use waterfall_rs::message::Message;
use waterfall_rs::statistics::Statistics;
use waterfall_rs::types::SampleType;

#[test]
fn test_ft8_receive() {
    let config = Configuration::default();

    let (sample_tx, mut sample_rx) = mpsc::channel::<Vec<SampleType>>(1024);
    let (plot_tx, mut plot_rx) = mpsc::channel::<Vec<SampleType>>(5);
    let (message_tx, mut message_rx) = mpsc::channel::<Box<dyn Message>>(5);

    let mut rx = Rx::new(plot_tx, message_tx, &config);
    let mut synth = InstantSynth::new(sample_tx, &config);

    let mut stats = Statistics::default();

    let mut message_received = false;
    for _ in 0..(1024 * 10) {
        synth.run(&config);

        if let Ok(samples) = sample_rx.try_recv() {
            rx.run(samples, &config, &mut stats);
        }

        while let Ok(_) = plot_rx.try_recv() {}

        if let Ok(_) = message_rx.try_recv() {
            println!("Message received for real");
            message_received = true;
            break;
        }
    }

    assert!(message_received);
}
