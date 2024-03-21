use tokio::sync::mpsc;

pub trait Message {
    fn payload(&self) -> String;
    fn mode(&self) -> String;
}

pub struct RttyMessage {
    message: String,
}

impl Message for RttyMessage {
    fn payload(&self) -> String {
        self.message.clone()
    }

    fn mode(&self) -> String {
        "RTTY".to_string()
    }
}

impl RttyMessage {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub type MessageSender = mpsc::Sender<Box<dyn Message>>;
pub type MessageReceiver = mpsc::Receiver<Box<dyn Message>>;
