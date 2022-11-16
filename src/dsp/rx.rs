use std::cmp::Ordering;

use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;
use wasm_timer::Instant;

use crate::configuration::Configuration;
use crate::dsp::aggregator::Aggregator;
use crate::dsp::correlator::{Correlator, OperandData};
use crate::message::{Ft8Message, MessageSender};
use crate::statistics::Statistics;
use crate::synth::ft8::SingleSymbol;
use crate::synth::ft8::SyncSignal;
use crate::synth::{Samples, Sine};
use crate::types::SampleType;
use crate::units::Frequency;

#[derive(Copy, Clone, Debug)]
enum RxMode {
    Sync1,
    MessageA(usize),
    Sync2,
    MessageB(usize),
    Done,
}

impl RxMode {
    pub fn is_sync(&self) -> bool {
        match self {
            RxMode::Sync1 | RxMode::Sync2 => true,
            _ => false,
        }
    }

    pub fn is_done(&self) -> bool {
        match self {
            RxMode::Done => true,
            _ => false,
        }
    }

    pub fn advance(self) -> Self {
        let next = match self {
            RxMode::Sync1 => RxMode::MessageA(29),
            RxMode::MessageA(s) => {
                if s == 0 {
                    RxMode::Sync2
                } else {
                    RxMode::MessageA(s - 1)
                }
            }
            RxMode::Sync2 => RxMode::MessageB(29),
            RxMode::MessageB(s) => {
                if s == 0 {
                    RxMode::Done
                } else {
                    RxMode::MessageB(s - 1)
                }
            }
            RxMode::Done => RxMode::Sync1,
        };

        next
    }

    pub fn reset(self) -> Self {
        let next = RxMode::Sync1;

        next
    }
}

impl Default for RxMode {
    fn default() -> Self {
        RxMode::Sync1
    }
}

pub struct Rx {
    symbols: Vec<OperandData>,
    correlator: Correlator,
    sync_correlator: Correlator,
    aggregator: Aggregator,
    sync_aggregator: Aggregator,
    sample_rate: Frequency,
    plot_sender: Sender<Vec<SampleType>>,
    message_sender: MessageSender,
    downsample_skip: usize,
    mode: RxMode,
    sync_data: OperandData,
}

impl Rx {
    pub fn new(
        plot_sender: Sender<Vec<SampleType>>,
        message_sender: MessageSender,
        config: &Configuration,
    ) -> Self {
        let mut symbols: Vec<OperandData> = Vec::with_capacity(8);

        let sample_rate_raw = config.audio_sample_rate;
        let sample_rate = Frequency::Hertz(sample_rate_raw as f32);
        let baseband_sample_rate = match sample_rate_raw {
            8000 => Frequency::Hertz(100.0),
            16000 => Frequency::Hertz(100.0),
            22050 => Frequency::Hertz(105.0),
            44100 => Frequency::Hertz(100.0),
            48000 => Frequency::Hertz(1000.0),
            96000 => Frequency::Hertz(100.0),
            _ => sample_rate,
        };

        let downsample_skip: usize = sample_rate_raw / (baseband_sample_rate.value() as usize);

        let carrier = Frequency::Hertz(0.0);

        let aggregator_len = (sample_rate.value() / 6.25) as usize;
        let aggregator = Aggregator::new(aggregator_len);
        let sync_aggregator = Aggregator::new(aggregator_len * 7 * 2);

        let buffer_len = (baseband_sample_rate.value() / 6.25) as usize;
        let correlator = Correlator::with_pow2_len(buffer_len);
        let sync_correlator = Correlator::with_pow2_len(buffer_len * 7);

        let SyncSignal(mut sync_signal) = SyncSignal::new(baseband_sample_rate);
        sync_signal.resize(sync_correlator.input_size(), 0.0);
        let sync_data = sync_correlator.prepare_rhs(&sync_signal);

        for symbol in 0..8 {
            let mut gen =
                SingleSymbol::with_amplitude(baseband_sample_rate, carrier, symbol as f32, 1.0);

            // let len: usize =
            //     (sample_rate.value() / (carrier.value() + (symbol as f32) * 6.25)) as usize;
            let syn: Vec<SampleType> = (0..correlator.output_size())
                .into_iter()
                .map(|_| gen.next())
                .collect();

            symbols.push(correlator.prepare_rhs(&syn));
        }
        Self {
            symbols,
            correlator,
            sync_correlator,
            aggregator,
            sync_aggregator,
            sample_rate,
            plot_sender,
            message_sender,
            downsample_skip,
            mode: RxMode::default(),
            sync_data,
        }
    }

    pub fn run(
        &mut self,
        new_samples: Vec<SampleType>,
        config: &Configuration,
        stats: &mut Statistics,
    ) {
        let sample_rate = Frequency::Hertz(config.audio_sample_rate as f32);
        if sample_rate.value() != self.sample_rate.value() {
            *self = Self::new(
                self.plot_sender.clone(),
                self.message_sender.clone(),
                config,
            );
        }

        let (aggregator, other_aggregator) = match self.mode.is_sync() {
            true => (&mut self.sync_aggregator, &mut self.aggregator),
            false => (&mut self.aggregator, &mut self.sync_aggregator),
        };
        aggregator.aggregate(new_samples);

        while let Some(mut samples) = aggregator.get_slice() {
            let now = Instant::now();

            // Bandpass
            // let mut bpf1 = BandPassFilter::from_frequency(
            //     config.tuner.lower_absolute(), // Low
            //     config.tuner.upper_absolute(), // High
            //     self.sample_rate,              // SampleRate
            // );
            // let mut bpf2 = BandPassFilter::from_frequency(
            //     config.tuner.lower_absolute(), // Low
            //     config.tuner.upper_absolute(), // High
            //     self.sample_rate,              // SampleRate
            // );
            // let mut bpf3 = BandPassFilter::from_frequency(
            //     config.tuner.lower_absolute(), // Low
            //     config.tuner.upper_absolute(), // High
            //     self.sample_rate,              // SampleRate
            // );
            // let bp1 = samples.into_iter().map(|sample| bpf1.next(sample));
            // let bp2 = bp1.map(|sample| bpf2.next(sample));
            // let bandpassed = bp2.map(|sample| bpf3.next(sample));

            // LO Mix
            // let if_carrier = config.tuner.carrier() - Frequency::Hertz(100.001);
            let if_carrier = config.tuner.carrier();
            let mut carrier = Sine::new(self.sample_rate, if_carrier);
            let mixed = samples.iter().map(|sample| sample * carrier.next());

            // Low Pass
            // let mut lpf = LowPassFilter::from_frequency(
            //     Frequency::Hertz(1000.0),
            //     self.sample_rate, // SampleRate
            // );
            // let low_passed = mixed.map(|sample| lpf.next(sample));

            // Collect into signal
            let signal: Vec<SampleType> = mixed.step_by(self.downsample_skip).collect();

            // if let Err(err) = self.plot_sender.try_send(sync_signal) {
            if let Err(err) = self.plot_sender.try_send(signal.clone()) {
                match err {
                    TrySendError::Full(_) => println!("Plot ui falling behind"),
                    TrySendError::Closed(_) => (),
                }
            }

            // Correlate
            if self.mode.is_sync() {
                let input_size = self.sync_correlator.input_size();
                let (value, position) =
                    (0..signal.len() / 2)
                        .into_iter()
                        .step_by(64)
                        .fold((0., 0), |max, start| {
                            let end = if start + input_size > signal.len() {
                                signal.len()
                            } else {
                                start + input_size
                            };

                            let lhs = self.sync_correlator.prepare_lhs(&signal[start..end]);
                            let (value, position) = self
                                .sync_correlator
                                .correlate_max_with_prepared(&lhs, &self.sync_data, true);

                            if value > max.0 {
                                (value, end)
                            } else {
                                max
                            }
                        });

                if value > 0.5 {
                    // Advance states and return data.
                    let return_samples = samples.split_off(position * self.downsample_skip);
                    aggregator.return_slice(return_samples);
                    other_aggregator.take_data(aggregator);
                    self.mode = self.mode.advance();
                } else {
                    self.mode = self.mode.reset();
                }
                println!("Max value of {} at {}", value, position);
            } else {
                let lhs = self.correlator.prepare_lhs(&signal);
                let _symbol = self
                    .symbols
                    .iter()
                    .map(|syn| {
                        self.correlator
                            .correlate_max_with_prepared(&lhs, syn, true)
                            .0
                    })
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                    .map(|(index, _)| index)
                    .unwrap();
                self.mode = self.mode.advance();
            }

            if self.mode.is_done() {
                println!("Message received");
                let message = Box::new(Ft8Message::new());
                if let Err(err) = self.message_sender.try_send(message) {
                    match err {
                        TrySendError::Full(_) => println!("Message receiver falling behind."),
                        TrySendError::Closed(_) => (),
                    }
                }
                self.mode.reset();
            }

            let elapsed = now.elapsed();
            stats.rx.push(elapsed);

            // println!("Decoded: {}", symbol);
            // let ft8_sym = Ft8Symbol::from(symbol);
            // if let Some(_message) = self.ft8_rx.next(ft8_sym) {
            //     println!("An FT8 message was received!");
            // }
        }
    }
}
