use std::fmt::Display;

pub enum ControlType {
    Letters,
    Figures,
    Null,
    Space,
    LineFeed,
    CarriageReturn,
}

#[derive(Copy, Clone, Default)]
pub enum SymbolState {
    #[default]
    Letters,
    Figures,
}

pub enum Symbol {
    Control(ControlType),
    Letter(char),
    Figure(char),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Control(ct) => match ct {
                ControlType::Null => '\0',
                ControlType::Space => ' ',
                ControlType::Letters => 'l',
                ControlType::Figures => 'f',
                ControlType::LineFeed => '\n',
                ControlType::CarriageReturn => '\r',
            },
            Self::Letter(c) => *c,
            Self::Figure(c) => *c,
        };
        write!(f, "{}", c)
    }
}

pub fn decode(val: u8, state: SymbolState) -> Symbol {
    match state {
        SymbolState::Letters => match val {
            0b11111 => Symbol::Control(ControlType::Letters),
            0b11011 => Symbol::Control(ControlType::Figures),
            0b00000 => Symbol::Control(ControlType::Null),
            0b00100 => Symbol::Control(ControlType::Space),
            0b01000 => Symbol::Control(ControlType::LineFeed),
            0b00010 => Symbol::Control(ControlType::CarriageReturn),
            0b00011 => Symbol::Letter('A'),
            0b11001 => Symbol::Letter('B'),
            0b01110 => Symbol::Letter('C'),
            0b01001 => Symbol::Letter('D'),
            0b00001 => Symbol::Letter('E'),
            0b01101 => Symbol::Letter('F'),
            0b11010 => Symbol::Letter('G'),
            0b10100 => Symbol::Letter('H'),
            0b00110 => Symbol::Letter('I'),
            0b01011 => Symbol::Letter('J'),
            0b01111 => Symbol::Letter('K'),
            0b10010 => Symbol::Letter('L'),
            0b11100 => Symbol::Letter('M'),
            0b01100 => Symbol::Letter('N'),
            0b11000 => Symbol::Letter('O'),
            0b10110 => Symbol::Letter('P'),
            0b10111 => Symbol::Letter('Q'),
            0b01010 => Symbol::Letter('R'),
            0b00101 => Symbol::Letter('S'),
            0b10000 => Symbol::Letter('T'),
            0b00111 => Symbol::Letter('U'),
            0b11110 => Symbol::Letter('V'),
            0b10011 => Symbol::Letter('W'),
            0b11101 => Symbol::Letter('X'),
            0b10101 => Symbol::Letter('Y'),
            0b10001 => Symbol::Letter('Z'),
            _ => unreachable!("values should be <= 0x1f"),
        },
        SymbolState::Figures => match val {
            0b11111 => Symbol::Control(ControlType::Letters),
            0b11011 => Symbol::Control(ControlType::Figures),
            0b00000 => Symbol::Control(ControlType::Null),
            0b00100 => Symbol::Control(ControlType::Space),
            0b01000 => Symbol::Control(ControlType::LineFeed),
            0b00010 => Symbol::Control(ControlType::CarriageReturn),
            0b00011 => Symbol::Letter('-'),
            0b11001 => Symbol::Letter('?'),
            0b01110 => Symbol::Letter(':'),
            0b01001 => Symbol::Letter('$'),
            0b00001 => Symbol::Letter('3'),
            0b01101 => Symbol::Letter('!'),
            0b11010 => Symbol::Letter('&'),
            0b10100 => Symbol::Letter('#'),
            0b00110 => Symbol::Letter('8'),
            0b01011 => Symbol::Letter('\''),
            0b01111 => Symbol::Letter('('),
            0b10010 => Symbol::Letter(')'),
            0b11100 => Symbol::Letter('.'),
            0b01100 => Symbol::Letter(','),
            0b11000 => Symbol::Letter('9'),
            0b10110 => Symbol::Letter('0'),
            0b10111 => Symbol::Letter('1'),
            0b01010 => Symbol::Letter('4'),
            0b00101 => Symbol::Letter('b'),
            0b10000 => Symbol::Letter('5'),
            0b00111 => Symbol::Letter('7'),
            0b11110 => Symbol::Letter(';'),
            0b10011 => Symbol::Letter('2'),
            0b11101 => Symbol::Letter('/'),
            0b10101 => Symbol::Letter('6'),
            0b10001 => Symbol::Letter('"'),
            _ => unreachable!("values should be <= 0x1f"),
        }
    }
}
