use egui::*;

use crate::configuration::Configuration;
use crate::filter_cutoff::{Carrier, FilterConnection, FilterCutoffLower, FilterCutoffUpper};
use crate::units::Frequency;

pub struct WaterfallTicks<'a> {
    config: &'a mut Configuration,
}

fn tick_interval(bandwidth: f32, pixel_width: f32, target: f32) -> (f32, f32) {
    let pixels_per_hz = pixel_width / bandwidth;
    let hz_per_target = target / pixels_per_hz;
    let digits = hz_per_target.log10().floor();
    let leading = (hz_per_target / 10.0_f32.powf(digits)).floor();

    let f_width: f32;

    if leading >= 5.0 {
        f_width = 5.0 * 10.0_f32.powf(digits);
    } else if leading >= 2.0 {
        f_width = 2.0 * 10.0_f32.powf(digits);
    } else {
        f_width = 1.0 * 10.0_f32.powf(digits);
    }

    (f_width, f_width * pixels_per_hz)
}

impl<'a> WaterfallTicks<'a> {
    pub fn new(config: &'a mut Configuration) -> Self {
        Self { config }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::bottom("tuner-axis")
            .frame(Frame::none().fill(ui.style().visuals.faint_bg_color))
            .default_height(48.0)
            .show_inside(ui, |ui| {
                let size = ui.available_size();

                let lower = self
                    .config
                    .freq_to_zoom_interval(self.config.tuner.lower_absolute());

                let upper = self
                    .config
                    .freq_to_zoom_interval(self.config.tuner.upper_absolute());

                let config2 = *self.config;
                FilterConnection::new(lower, upper).ui(ui);
                if lower > 0.0 {
                    FilterCutoffLower::new(&mut self.config.tuner.lower, &config2, size.x).ui(ui);
                }
                if upper < 1.0 {
                    FilterCutoffUpper::new(&mut self.config.tuner.upper, &config2, size.x).ui(ui);
                }

                Carrier::new(&mut self.config.tuner.carrier, &config2, size.x).ui(ui);
            });

        egui::TopBottomPanel::bottom("time-axis")
            .frame(Frame::none().fill(ui.style().visuals.faint_bg_color))
            .default_height(34.0)
            .show_inside(ui, |ui| {
                let color = ui.style().visuals.text_color();
                let stroke = Stroke { width: 1.0, color };
                let size = ui.available_size();
                let rect = ui.max_rect();
                let (response, painter) = ui.allocate_painter(size, Sense::hover());
                if response.hovered() {
                    if let Some(pos) = response.hover_pos() {
                        let interval_pos = pos.x / size.x;
                        let hover_freq = self.config.zoomed_interval_to_hz(interval_pos);
                        response.on_hover_text_at_pointer(hover_freq.to_string());
                    }
                }

                // painter.rect_filled(rect, Rounding::none(), ui.style().visuals.faint_bg_color);

                let displayed_bandwidth = self.config.displayed_bandwidth();
                let start_hz = self.config.start_hz();

                let (f_width, pixel_width) = tick_interval(
                    displayed_bandwidth,
                    rect.width(),
                    200.0_f32.min(rect.width()),
                );

                for i in 0..(10.0 * rect.width() / pixel_width + 1.0) as usize {
                    let x = rect.left() + (i as f32) * pixel_width / 10.0;
                    painter.line_segment(
                        [
                            Pos2 { x, y: rect.top() },
                            Pos2 {
                                x,
                                y: rect.top() + 5.0,
                            },
                        ],
                        stroke,
                    );
                }

                for i in 0..(2.0 * rect.width() / pixel_width + 1.0) as usize {
                    let x = rect.left() + (i as f32) * pixel_width / 2.0;
                    painter.line_segment(
                        [
                            Pos2 { x, y: rect.top() },
                            Pos2 {
                                x,
                                y: rect.top() + 10.0,
                            },
                        ],
                        stroke,
                    );
                }

                for i in 0..(rect.width() / pixel_width + 1.0) as usize {
                    let x = rect.left() + (i as f32) * pixel_width;
                    painter.line_segment(
                        [
                            Pos2 { x, y: rect.top() },
                            Pos2 {
                                x,
                                y: rect.top() + 15.0,
                            },
                        ],
                        stroke,
                    );
                }

                for i in 0..(rect.width() / pixel_width + 1.0) as usize {
                    let x = rect.left() + (i as f32) * pixel_width;

                    let mut align: Align2 = Align2::CENTER_TOP;
                    if i == 0 {
                        align = Align2::LEFT_TOP;
                    } else if i == (rect.width() / pixel_width) as usize {
                        align = Align2::RIGHT_TOP;
                    }

                    let frequency = Frequency::Hertz(i as f32 * f_width + start_hz);

                    painter.text(
                        Pos2 {
                            x,
                            y: rect.top() + 18.0,
                        },
                        align,
                        format!("{}", frequency),
                        FontId::proportional(14.0),
                        color,
                    );
                }
            });
    }
}
