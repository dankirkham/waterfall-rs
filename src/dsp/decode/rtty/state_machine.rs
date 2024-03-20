use crate::units::Frequency;
use crate::dsp::decode::rtty::symbols::{decode, ControlType, SymbolState};

use super::symbols::Symbol;

#[derive(Default)]
enum State {
    #[default]
    Reset,
    WaitForStart,
    WaitForAssertStart(u32),
    Bits(u32, u32),
    WaitForStop(u32),
}

pub struct StateMachine {
    full_wait: u32,
    state: State,
    data: u8,
    symbol_state: SymbolState,
}

impl StateMachine {
    pub fn new(sample_rate: Frequency, baudrate: Frequency) -> Self {
        let full_wait = sample_rate.value() / baudrate.value();
        let full_wait = full_wait as u32;

        Self {
            full_wait,
            state: Default::default(),
            data: Default::default(),
            symbol_state: Default::default(),
        }
    }

    pub fn update(&mut self, mark: bool) {
        match self.state {
            State::Reset => {
                self.data = 0;
                if mark {
                    self.state = State::WaitForStart;
                }
            }
            State::WaitForStart => {
                if !mark {
                    self.state = State::WaitForAssertStart(self.full_wait / 2);
                }
            }
            State::WaitForAssertStart(wait) => {
                if wait != 0 {
                    self.state = State::WaitForAssertStart(wait - 1);
                } else {
                    if !mark {
                        self.state = State::Bits(self.full_wait, 5);
                    } else {
                        println!("Start bit failed");
                        self.state = State::Reset;
                    }
                }
            }
            State::Bits(wait, mut bits) => {
                if wait != 0 {
                    self.state = State::Bits(wait - 1, bits);
                } else {
                    if mark {
                        self.data = 0x80 | (self.data >> 1);
                    } else {
                        self.data = self.data >> 1;
                    }

                    bits -= 1;
                    if bits == 0 {
                        self.state = State::WaitForStop(self.full_wait);
                    } else {
                        self.state = State::Bits(self.full_wait, bits);
                    }
                }
            }
            State::WaitForStop(wait) => {
                if wait != 0 {
                    self.state = State::WaitForStop(wait - 1);
                } else {
                    self.data = self.data >> 3;
                    let symbol = decode(self.data, self.symbol_state);
                    match symbol {
                        Symbol::Control(ref ct) => {
                            match ct {
                                ControlType::Letters => self.symbol_state = SymbolState::Letters,
                                ControlType::Figures => self.symbol_state = SymbolState::Figures,
                                _ => print!("{}", symbol),
                            }
                        },
                        _ => print!("{}", symbol),
                    }
                    if mark {
                        self.state = State::Reset;
                    } else {
                        eprintln!("Stop bit failed");
                        self.state = State::Reset;
                    }
                }
            }
        }
    }
}
