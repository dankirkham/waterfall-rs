#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Symbol {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl From<usize> for Symbol {
    fn from(int: usize) -> Self {
        match int {
            0 => Symbol::Zero,
            1 => Symbol::One,
            2 => Symbol::Two,
            3 => Symbol::Three,
            4 => Symbol::Four,
            5 => Symbol::Five,
            6 => Symbol::Six,
            7 => Symbol::Seven,
            _ => panic!("Invalid symbol int"),
        }
    }
}

impl Into<Vec<bool>> for Symbol {
    fn into(self) -> Vec<bool> {
        match self {
            Symbol::Zero => vec![false, false, false],
            Symbol::One => vec![false, false, true],
            Symbol::Two => vec![false, true, true],
            Symbol::Three => vec![false, true, false],
            Symbol::Four => vec![true, true, false],
            Symbol::Five => vec![true, false, false],
            Symbol::Six => vec![true, false, true],
            Symbol::Seven => vec![true, true, true],
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum SyncState {
    Three,
    One,
    Four,
    Zero,
    Six,
    Five,
    Two,
}

impl Default for SyncState {
    fn default() -> Self {
        SyncState::Three
    }
}

impl SyncState {
    /// Go to next sync state on symbols
    /// Returns None when state machine is in progress
    /// Returns false when sync has failed.
    /// Returns true when sync is finished.
    pub fn next(&mut self, symbol: &Symbol) -> Option<bool> {
        let mut done = None;
        *self = match *self {
            SyncState::Three => if *symbol == Symbol::Three {
                SyncState::One
            } else {
                done = Some(false);
                SyncState::Three
            },
            SyncState::One => if *symbol == Symbol::One {
                println!("Two deep into Sync");
                SyncState::Four
            } else {
                done = Some(false);
                SyncState::Three
            },
            SyncState::Four => if *symbol == Symbol::Four {
                println!("Three deep into Sync");
                SyncState::Zero
            } else {
                done = Some(false);
                SyncState::Three
            },
            SyncState::Zero => if *symbol == Symbol::Zero {
                SyncState::Six
            } else {
                done = Some(false);
                SyncState::Three
            },
            SyncState::Six => if *symbol == Symbol::Six {
                SyncState::Five
            } else {
                done = Some(false);
                SyncState::Three
            },
            SyncState::Five => if *symbol == Symbol::Five {
                SyncState::Two
            } else {
                done = Some(false);
                SyncState::Three
            },
            SyncState::Two => if *symbol == Symbol::Two {
                println!("Got a Sync!");
                done = Some(true);
                SyncState::Three
            } else {
                done = Some(false);
                SyncState::Three
            },
        };

        return done;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Sync1,
    MessageA, // [0, 28]
    Sync2,
    MessageB, // [29, 57]
    Sync3,
}

impl Default for State {
    fn default() -> Self {
        State::Sync1
    }
}

#[derive(Default)]
pub struct Ft8Rx {
    state: State,
    sync_state: SyncState,
    bits: Vec<bool>,
    message_symbols: u8,
}

impl Ft8Rx {
    pub fn next(&mut self, symbol: Symbol) -> Option<Vec<bool>> {
        let mut message: Option<Vec<bool>> = None;
        match self.state {
            State::Sync1 => if let Some(result) = self.sync_state.next(&symbol) {
                if result {
                    self.state = State::MessageA;
                    self.message_symbols = 29;
                }
            },
            State::MessageA => {
                self.push_symbol(symbol);
                self.message_symbols -= 1;
                if self.message_symbols == 0 {
                    self.state = State::Sync2;
                }
            },
            State::Sync2 => if let Some(result) = self.sync_state.next(&symbol) {
                if result {
                    self.state = State::MessageB;
                    self.message_symbols = 29;
                } else {
                    self.reset();
                }
            },
            State::MessageB => {
                self.push_symbol(symbol);
                self.message_symbols -= 1;
                if self.message_symbols == 0 {
                    self.state = State::Sync3;
                }
            },
            State::Sync3 => if let Some(result) = self.sync_state.next(&symbol) {
                if result {
                    let mut bits: Vec<bool> = Vec::with_capacity(58);
                    std::mem::swap(&mut bits, &mut self.bits);
                    message = Some(bits);
                }
                self.reset();
            },
        }
        message
    }

    fn reset(&mut self) {
        self.bits.clear();
        self.state = State::Sync1;
    }

    fn push_symbol(&mut self, symbol: Symbol) {
        let mut bits: Vec<bool> = symbol.into();
        self.bits.append(&mut bits);
    }
}
