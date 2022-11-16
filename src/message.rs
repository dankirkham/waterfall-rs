use tokio::sync::mpsc;

pub trait Message {
    fn value(&self) -> String;
}

pub struct Ft8Message {
    message: String,
}

impl Message for Ft8Message {
    fn value(&self) -> String {
        self.message.clone()
    }
}

impl Ft8Message {
    pub fn new() -> Self {
        Self {
            message: "An FT8 message.".to_owned(),
        }
    }
}

pub type MessageSender = mpsc::Sender<Box<dyn Message>>;
pub type MessageReceiver = mpsc::Receiver<Box<dyn Message>>;
