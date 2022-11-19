#[derive(Copy, Clone, Debug)]
pub enum RxMode {
    Sync1,
    MessageA(usize),
    Sync2,
    MessageB(usize),
    Sync3,
    Done,
}

impl RxMode {
    pub fn is_sync(&self) -> bool {
        match self {
            RxMode::Sync1 | RxMode::Sync2 | RxMode::Sync3 => true,
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
                    RxMode::Sync3
                } else {
                    RxMode::MessageB(s - 1)
                }
            }
            RxMode::Sync3 => RxMode::Done,
            RxMode::Done => RxMode::Sync1,
        };

        // dbg!(next);
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
