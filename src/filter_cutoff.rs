use egui::*;

use crate::configuration::Configuration;
use crate::units::Frequency;

pub struct FilterCutoffLower<'a> {
    f: &'a mut Frequency,
    config: &'a Configuration,
    width: f32,
}

impl<'a> FilterCutoffLower<'a> {
    pub fn new(f: &'a mut Frequency, config: &'a Configuration, width: f32) -> Self {
        Self { f, config, width }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let WIDTH = 10.0;
        let rect = ui.max_rect();

        let x = self
            .config
            .freq_to_zoom_interval(self.config.tuner.carrier + *self.f);
        let pos = x * rect.width();

        let rect = Rect::from_x_y_ranges(
            0_f32.max(rect.left())..=pos.min(rect.right()),
            rect.top()..=(rect.top() + rect.height() / 2.0),
        );
        ui.allocate_ui_at_rect(rect, |ui| {
            let size = ui.available_size();
            let rect = ui.max_rect();
            let (response, painter) = ui.allocate_painter(size, Sense::drag());

            let mut color: Color32;
            if response.hovered() || response.dragged() {
                color = ui.style().visuals.strong_text_color();
            } else {
                color = ui.style().visuals.text_color();
            }

            let stroke = Stroke { width: 1.0, color };
            let bottom = rect.right_bottom() - Vec2::new(WIDTH, 0.0);
            painter.line_segment([rect.right_top(), bottom], stroke);
            if response.dragged() {
                if let Some(pos) = response.hover_pos() {
                    let interval_pos = pos.x / self.width;
                    let abs_hz = self.config.zoomed_interval_to_hz(interval_pos);
                    let rel_hz = abs_hz - self.config.tuner.carrier;
                    *self.f = rel_hz;
                }
            }
        });
    }
}

pub struct FilterCutoffUpper<'a> {
    f: &'a mut Frequency,
    config: &'a Configuration,
    width: f32,
}

impl<'a> FilterCutoffUpper<'a> {
    pub fn new(f: &'a mut Frequency, config: &'a Configuration, width: f32) -> Self {
        Self { f, config, width }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let WIDTH = 10.0;
        let rect = ui.max_rect();

        let x = self
            .config
            .freq_to_zoom_interval(self.config.tuner.carrier + *self.f);
        let pos = x * rect.width();

        let rect = Rect::from_x_y_ranges(
            pos.max(rect.left())..=rect.right(),
            rect.top()..=(rect.top() + rect.height() / 2.0),
        );
        ui.allocate_ui_at_rect(rect, |ui| {
            let size = ui.available_size();
            let rect = ui.max_rect();
            let (response, painter) = ui.allocate_painter(size, Sense::drag());

            let mut color: Color32;
            if response.hovered() || response.dragged() {
                color = ui.style().visuals.strong_text_color();
            } else {
                color = ui.style().visuals.text_color();
            }

            let stroke = Stroke { width: 1.0, color };
            let bottom = rect.left_bottom() + Vec2::new(WIDTH, 0.0);
            painter.line_segment([rect.left_top(), bottom], stroke);
            if response.dragged() {
                if let Some(pos) = response.hover_pos() {
                    let interval_pos = pos.x / self.width;
                    let abs_hz = self.config.zoomed_interval_to_hz(interval_pos);
                    let rel_hz = abs_hz - self.config.tuner.carrier;
                    *self.f = rel_hz;
                }
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

        let rect =
            Rect::from_x_y_ranges(pos1..=pos2, rect.top()..=(rect.top() + rect.height() / 2.0));
        ui.allocate_ui_at_rect(rect, |ui| {
            let size = ui.available_size();
            let rect = ui.max_rect();
            let (_, painter) = ui.allocate_painter(size, Sense::drag());

            let color = ui.style().visuals.text_color();

            let stroke = Stroke { width: 1.0, color };
            // painter.rect_filled(rect, Rounding::none(), ui.style().visuals.code_bg_color);
            painter.line_segment([rect.left_top(), rect.right_top()], stroke);
        });
    }
}

pub struct Carrier<'a> {
    f: &'a mut Frequency,
    config: &'a Configuration,
    width: f32,
}

impl<'a> Carrier<'a> {
    pub fn new(f: &'a mut Frequency, config: &'a Configuration, width: f32) -> Self {
        Self { f, config, width }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let WIDTH = 30.0;
        let rect = ui.max_rect();

        let x = self.config.freq_to_zoom_interval(*self.f);
        let pos = x * rect.width();

        let rect = Rect::from_center_size(
            Pos2::new(pos, rect.center().y),
            Vec2::new(WIDTH, rect.height()),
        );
        ui.allocate_ui_at_rect(rect, |ui| {
            let size = ui.available_size();
            let rect = ui.max_rect();
            let (response, painter) = ui.allocate_painter(size, Sense::drag());

            let mut color: Color32;
            if response.hovered() || response.dragged() {
                color = ui.style().visuals.strong_text_color();
            } else {
                color = ui.style().visuals.text_color();
            }

            let stroke = Stroke { width: 3.0, color };
            painter.line_segment([rect.center_top(), rect.center_bottom()], stroke);
            if response.dragged() {
                if let Some(pos) = response.hover_pos() {
                    let interval_pos = pos.x / self.width;
                    let hz = self.config.zoomed_interval_to_hz(interval_pos);
                    *self.f = hz;
                }
            }
        });
    }
}
