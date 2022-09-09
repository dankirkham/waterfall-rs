use crate::synth::ft8::Symbol;
use crate::synth::Samples;
use crate::units::Frequency;
use crate::synth::ft8::message::{Message, sync_sequence};

#[derive(Copy, Clone, Debug, PartialEq)]
enum MessageState {
    Startup,
    Sync1,
    MessageA,
    Sync2,
    MessageB,
    Sync3,
}

impl MessageState {
    pub fn next(self) -> Self {
        match self {
            MessageState::Startup => MessageState::Sync1,
            MessageState::Sync1 => MessageState::MessageA,
            MessageState::MessageA => MessageState::Sync2,
            MessageState::Sync2 => MessageState::MessageB,
            MessageState::MessageB => MessageState::Sync3,
            MessageState::Sync3 => MessageState::Sync1,
        }
    }

    pub fn is_message(&self) -> bool {
        match *self {
            MessageState::Startup => true,
            MessageState::MessageA => true,
            MessageState::MessageB => true,
            MessageState::Sync1 => false,
            MessageState::Sync2 => false,
            MessageState::Sync3 => false,
        }
    }
}

pub struct Ft8 {
    sample_rate: Frequency,
    sample: u64,
    symbol_count: usize,
    synth: Symbol,
    symbols: Vec<u8>,
    sync_symbols: Vec<u8>,
    state: MessageState,
}

impl Ft8 {
    pub fn new(sample_rate: Frequency, carrier: Frequency) -> Self {
        let signaling_interval = 0.160;
        let amplitude = 0.005;
        let sync_symbols = sync_sequence();
        let symbols: Vec<u8> = Message::default().into();
        let synth = Symbol::with_amplitude(sample_rate, carrier, symbols[0] as f32, amplitude);
        let state = MessageState::Startup;
        let symbol_count = 14; // Give time for receiver to get ready.

        Self {
            sample_rate,
            sample: (sample_rate.value() * signaling_interval) as u64,
            symbol_count,
            synth,
            symbols,
            sync_symbols,
            state,
        }
    }
}

impl Samples for Ft8 {
    fn next(&mut self) -> f32 {
        if self.sample == 0 {
            let signaling_interval = 0.160;
            self.sample = (self.sample_rate.value() * signaling_interval) as u64;

            self.symbol_count -= 1;
            if self.symbol_count == 0 {
                self.state = self.state.next();
                match self.state.is_message() {
                    true => self.symbol_count = self.symbols.len() / 2,
                    false => self.symbol_count = self.sync_symbols.len(),
                }
            }

            // println!("--> Symbol:  {}", self.symbol);

            let symbol = match self.state.is_message() {
                true => self.symbols[self.symbols.len() - self.symbol_count] as f32,
                false => self.sync_symbols[self.sync_symbols.len() - self.symbol_count] as f32,
            };
            self.synth.set_symbol(symbol);
        }

        self.sample -= 1;

        self.synth.next()
    }
}
