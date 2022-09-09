pub struct Bump<'a> {
    value: &'a mut f32,
    label: String,
}

impl<'a> Bump<'a> {
    pub fn new(value: &'a mut f32, label: String) -> Self {
        Self { value, label }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // if ui.add(egui::Button::new("➖➖➖")).clicked() {
            //     if *self.value % 1.0 != 0.0 {
            //         *self.value = self.value.floor();
            //     }
            //     *self.value -= 100.0;
            // }
            // if ui.add(egui::Button::new("➖➖")).clicked() {
            //     if *self.value % 1.0 != 0.0 {
            //         *self.value = self.value.floor();
            //     }
            //     *self.value -= 10.0;
            // }
            if ui.add(egui::Button::new("➖")).clicked() {
                if *self.value % 1.0 != 0.0 {
                    *self.value = self.value.floor();
                } else {
                    *self.value -= 1.0;
                }
            }
            ui.add(egui::DragValue::new(self.value));
            if ui.add(egui::Button::new("➕")).clicked() {
                if *self.value % 1.0 != 0.0 {
                    *self.value = self.value.ceil();
                } else {
                    *self.value += 1.0;
                }
            }
            // if ui.add(egui::Button::new("➕➕")).clicked() {
            //     if *self.value % 1.0 != 0.0 {
            //         *self.value = self.value.ceil();
            //     }
            //     *self.value += 10.0;
            // }
            // if ui.add(egui::Button::new("➕➕➕")).clicked() {
            //     if *self.value % 1.0 != 0.0 {
            //         *self.value = self.value.ceil();
            //     }
            //     *self.value += 100.0;
            // }
            ui.label(self.label.to_string());
        });
    }
}
