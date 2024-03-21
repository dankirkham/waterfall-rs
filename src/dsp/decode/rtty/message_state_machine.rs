use crate::dsp::decode::rtty::symbols::*;
use crate::message::RttyMessage;
use crate::units::Frequency;

#[derive(Default, PartialEq)]
enum State {
    #[default]
    Idle,
    Message,
}

pub struct MessageStateMachine {
    symbol_state: SymbolState,
    state: State,
    timeout: u32,
    counter: u32,
    message: String,
}

impl MessageStateMachine {
    pub fn new(sample_rate: Frequency) -> Self {
        let timeout = (sample_rate.value() * 0.300) as u32;
        dbg!(&timeout);

        Self {
            symbol_state: Default::default(),
            state: Default::default(),
            message: Default::default(),
            timeout,
            counter: timeout - 1,
        }
    }

    pub fn update(&mut self, symbol: Option<u8>) -> Option<RttyMessage> {
        if let Some(symbol) = symbol {
            let symbol = decode(symbol, self.symbol_state);
            if let Symbol::Control(ref ct) = symbol {
                match ct {
                    ControlType::Letters => {
                        self.symbol_state = SymbolState::Letters;
                        self.state = State::Message;
                        self.counter = self.timeout;
                    }
                    ControlType::Figures => {
                        self.symbol_state = SymbolState::Figures;
                        self.state = State::Message;
                        self.counter = self.timeout;
                    }
                    ControlType::LineFeed | ControlType::CarriageReturn => {
                        if self.state == State::Message {
                            self.state = State::Idle;
                            let message = RttyMessage::new(self.message.clone());
                            self.message.clear();
                            return Some(message);
                        }
                    }
                    ControlType::Space | ControlType::Null => {
                        if self.state == State::Message {
                            self.counter = self.timeout;
                            self.message.push(symbol.char().unwrap());
                        }
                    }
                }
            } else {
                self.state = State::Message;
                self.counter = self.timeout;
                if let Some(c) = symbol.char() {
                    self.message.push(c);
                }
            }
        } else {
            match self.state {
                State::Message => {
                    self.counter -= 1;
                    if self.counter == 0 {
                        self.state = State::Idle;
                        let message = RttyMessage::new(self.message.clone());
                        self.message.clear();
                        return Some(message);
                    }
                }
                State::Idle => (),
            }
        }
        None
    }
}
