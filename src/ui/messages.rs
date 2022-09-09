pub struct Messages;

impl Messages {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Received messages will go here.");
    }
}
