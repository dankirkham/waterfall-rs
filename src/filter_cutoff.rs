use egui::*;

pub struct FilterCutoff {
    x: f32,
}

impl FilterCutoff {
    pub fn new(x: f32) -> Self {
        Self { x }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        let WIDTH = 10.0;
        let rect = ui.max_rect();
        let pos = self.x * rect.width();

        let rect = Rect::from_center_size(Pos2::new(pos, rect.center().y), Vec2::new(WIDTH, rect.height()));
        ui.allocate_ui_at_rect(rect, |ui| {
            let size = ui.available_size();
            let rect = ui.max_rect();
            let (response, painter) = ui.allocate_painter(size, Sense::drag());

            let mut color: Color32;
            if response.hovered() {
                color = ui.style().visuals.strong_text_color();
            } else {
                color = ui.style().visuals.text_color();
            }

            let stroke = Stroke { width: 1.0, color };
            painter.line_segment([rect.center_top(), rect.center_bottom()], stroke);
            if response.dragged {
                println!("Dragging {} -> {}", self.x, response.drag_delta().x);
            }
        });
    }
}

pub struct FilterConnection {
    x1: f32,
    x2: f32,
}

impl FilterConnection {
    pub fn new(x1: f32, x2: f32) -> Self {
        Self { x1, x2 }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        let rect = ui.max_rect();
        let pos1 = self.x1 * rect.width();
        let pos2 = self.x2 * rect.width();
        let width = pos2 - pos1;

        let rect = Rect::from_x_y_ranges(
            pos1..=pos2,
            rect.top()..=rect.bottom()
        );
        ui.allocate_ui_at_rect(rect, |ui| {
            let size = ui.available_size();
            let rect = ui.max_rect();
            let (_, painter) = ui.allocate_painter(size, Sense::drag());

            let color = ui.style().visuals.text_color();

            let stroke = Stroke { width: 1.0, color };
            painter.rect_filled(rect, Rounding::none(), ui.style().visuals.code_bg_color);
            // painter.line_segment([rect.left_center(), rect.right_center()], stroke);
        });
    }
}
