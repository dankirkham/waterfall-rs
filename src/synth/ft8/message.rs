use rand::{thread_rng, Rng};
use rand::distributions::Standard;

pub fn sync_sequence() -> Vec<u8> {
    vec![3, 1, 4, 0, 6, 5, 2]
}

#[derive(Default)]
struct Callsign28;
impl Into<Vec<bool>> for Callsign28 {
    fn into(self) -> Vec<bool> {
        let mut rng = thread_rng();
        let v: Vec<bool> = thread_rng().sample_iter(&Standard).take(28).collect();
        v
        // vec![false; 28]
    }
}

#[derive(Default)]
struct Rover1;
impl Into<Vec<bool>> for Rover1 {
    fn into(self) -> Vec<bool> {
        let mut rng = thread_rng();
        let v: Vec<bool> = thread_rng().sample_iter(&Standard).take(1).collect();
        v
        // vec![false; 1]
    }
}

#[derive(Default)]
struct Grid15;
impl Into<Vec<bool>> for Grid15 {
    fn into(self) -> Vec<bool> {
        let mut rng = thread_rng();
        let v: Vec<bool> = thread_rng().sample_iter(&Standard).take(15).collect();
        v
        // vec![false; 15]
    }
}

#[derive(Default)]
struct Roger1;
impl Into<Vec<bool>> for Roger1 {
    fn into(self) -> Vec<bool> {
        let mut rng = thread_rng();
        let v: Vec<bool> = thread_rng().sample_iter(&Standard).take(1).collect();
        v
        // vec![false; 1]
    }
}

#[derive(Default)]
struct Checksum14;
impl Into<Vec<bool>> for Checksum14 {
    fn into(self) -> Vec<bool> {
        let mut rng = thread_rng();
        let v: Vec<bool> = thread_rng().sample_iter(&Standard).take(97).collect();
        v
        // vec![false; 97] // 14 + 83 bits for forward error correction
    }
}

#[derive(Default)]
pub struct StdMsgFields {
    call1: Callsign28,
    rover1: Rover1,
    call2: Callsign28,
    rover2: Rover1,
    grid: Grid15,
    roger: Roger1,
}

pub enum Message {
    FreeText,
    Dxpedition,
    FieldDay1,
    FieldDay2,
    Telemetry,
    StdMsg(StdMsgFields),
    EuVhf1,
    RttyRu,
    NonStdCall,
    EuVhf2,
}

impl Default for Message {
    fn default() -> Self {
        let fields = StdMsgFields::default();
        Message::StdMsg(fields)
    }
}

impl Into<Vec<bool>> for Message {
    fn into(self) -> Vec<bool> {
        match self {
            Message::StdMsg(fields) => {
                let mut bits: Vec<bool> = Vec::with_capacity(82);
                // i3
                bits.push(false);
                bits.push(false);
                bits.push(true);

                let mut call1: Vec<bool> = fields.call1.into();
                bits.append(&mut call1);

                let mut rover1: Vec<bool> = fields.rover1.into();
                bits.append(&mut rover1);

                let mut call2: Vec<bool> = fields.call2.into();
                bits.append(&mut call2);

                let mut rover2: Vec<bool> = fields.rover2.into();
                bits.append(&mut rover2);

                let mut grid: Vec<bool> = fields.grid.into();
                bits.append(&mut grid);

                let mut roger: Vec<bool> = fields.roger.into();
                bits.append(&mut roger);

                let mut checksum: Vec<bool> = Checksum14::default().into();
                bits.append(&mut checksum);

                bits
            },
            _ => todo!("Implement other message types"),
        }
    }
}

impl Into<Vec<u8>> for Message {
    fn into(self) -> Vec<u8> {
        let bits: Vec<bool> = self.into();

        bits
            .chunks(3)
            .map(|s| {
                ((s[2] as u8) << 2) |
                ((s[1] as u8) << 1) |
                ((s[0] as u8) << 0)
            })
            .collect()
    }
}
