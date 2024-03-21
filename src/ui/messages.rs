use crate::messages::MessageCollector;

pub struct Messages<'a> {
    data: &'a MessageCollector,
}

impl<'a> Messages<'a> {
    pub fn new(data: &'a MessageCollector) -> Self {
        Self { data }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                if self.data.data.len() == 0 {
                    ui.label("Received messages will appear here.");
                }
                for message in self.data.data.iter() {
                    ui.label(format!("{}: {}", message.mode(), message.payload()));
                }
            });
    }
}
