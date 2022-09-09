#[derive(Default)]
pub struct Messages;

impl Messages {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Received messages will go here.");
    }
}
