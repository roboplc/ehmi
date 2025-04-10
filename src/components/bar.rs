use core::fmt;
use std::ops::RangeInclusive;

use egui::{vec2, Align2, Color32, FontId, Pos2, Rect, Response, RichText, Stroke, Ui, Vec2};

use crate::colors::{get_text_color, GRAY, GRAY_DARK, SUCCESS};

/// Horizontal or vertical bar component
pub struct Bar {
    text: String,
    value: f32,
    font_size: f32,
    label_size: f32,
    bar_size: f32,
    fg_color: Color32,
    min: f32,
    max: f32,
    vertical: Option<f32>,
}

impl Bar {
    /// Create a new bar
    pub fn new<V>(value: V) -> Self
    where
        V: Into<f32>,
    {
        Self {
            text: <_>::default(),
            value: value.into(),
            font_size: 16.0,
            label_size: 10.0,
            bar_size: 5.0,
            fg_color: SUCCESS,
            min: 0.0,
            max: 100.0,
            vertical: None,
        }
    }

    /// Set the range of the bar
    pub fn range(mut self, range: RangeInclusive<f32>) -> Self {
        self.min = *range.start();
        self.max = *range.end();
        self
    }

    /// Set the bar vertical
    pub fn vertical(mut self, max_width: f32) -> Self {
        self.vertical = Some(max_width);
        self
    }

    /// Set the bar text
    pub fn text(mut self, text: impl fmt::Display) -> Self {
        self.text = text.to_string();
        self
    }

    /// Set the bar text font size
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Set the bar labels font size
    pub fn label_size(mut self, font_size: f32) -> Self {
        self.label_size = font_size;
        self
    }

    /// Sets the bar size (width for horizontal, height for vertical)
    pub fn bar_size(mut self, size: f32) -> Self {
        self.bar_size = size;
        self
    }

    /// Sets the bar foreground color
    pub fn fg_color(mut self, color: Color32) -> Self {
        self.fg_color = color;
        self
    }

    fn vertical_ui(self, ui: &mut Ui, vertical_size: f32, value: f32) -> Response {
        const HEIGHT: f32 = 240.0;
        const VALUE_OFFSET: f32 = 16.0;
        const LABEL_MARGIN: f32 = 4.0;

        let total_width = self.bar_size + VALUE_OFFSET + vertical_size;
        let total_height = HEIGHT + (LABEL_MARGIN + self.label_size) * 2.0;

        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(total_width, total_height), egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let therm_top = rect.min.y + self.label_size + LABEL_MARGIN;
            let therm_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, therm_top),
                Vec2::new(self.bar_size, HEIGHT),
            );

            let bg_color = ui.style().visuals.clone().extreme_bg_color;

            painter.rect(
                therm_rect,
                3.0,
                bg_color,
                Stroke::NONE,
                egui::StrokeKind::Inside,
            );

            let fill_height = therm_rect.height() * (value - self.min) / (self.max - self.min);
            let fill_rect = Rect::from_min_size(
                Pos2::new(therm_rect.min.x, therm_rect.max.y - fill_height),
                Vec2::new(therm_rect.width(), fill_height),
            );

            painter.rect_filled(fill_rect, 2.0, self.fg_color);

            let center_x = therm_rect.center().x;

            let label_color = if ui.visuals().dark_mode {
                GRAY
            } else {
                GRAY_DARK
            };

            painter.text(
                Pos2::new(center_x, therm_rect.min.y - LABEL_MARGIN),
                Align2::CENTER_BOTTOM,
                self.max,
                FontId::proportional(self.label_size),
                label_color,
            );

            painter.text(
                Pos2::new(center_x, therm_rect.max.y + LABEL_MARGIN),
                Align2::CENTER_TOP,
                self.min,
                FontId::proportional(self.label_size),
                label_color,
            );

            let text_color = get_text_color(ui);

            painter.text(
                Pos2::new(therm_rect.max.x + VALUE_OFFSET, therm_rect.center().y),
                Align2::LEFT_CENTER,
                self.text,
                FontId::proportional(self.font_size),
                text_color,
            );
        }

        response
    }
}

impl egui::Widget for Bar {
    fn ui(self, ui: &mut Ui) -> Response {
        let value = self.value.clamp(self.min, self.max);
        if let Some(vertical_size) = self.vertical {
            return self.vertical_ui(ui, vertical_size, value);
        }
        let bar_width = 180.0;
        let label_color = if ui.visuals().dark_mode {
            GRAY
        } else {
            GRAY_DARK
        };
        let text_color = get_text_color(ui);
        let painter = ui.painter();
        let min_str = format!("{}", self.min);
        let max_str = format!("{}", self.max);

        let min_text = RichText::new(&min_str)
            .color(label_color)
            .size(self.label_size);
        let max_text = RichText::new(&max_str)
            .color(label_color)
            .size(self.label_size);
        let text = RichText::new(&self.text)
            .color(text_color)
            .size(self.font_size);

        let gallery_min_label =
            painter.layout_no_wrap(min_str, FontId::proportional(self.label_size), label_color);
        let gallery_max_label =
            painter.layout_no_wrap(max_str, FontId::proportional(self.label_size), label_color);
        let text_width = painter
            .layout_no_wrap(
                self.text.to_string(),
                FontId::proportional(self.font_size),
                text_color,
            )
            .size()
            .x;

        let min_label_width = gallery_min_label.size().x;
        let max_label_width = gallery_max_label.size().x;
        let total_width = min_label_width + bar_width + max_label_width;
        let line_height = self.bar_size.max(self.label_size);
        let label_offset = (line_height - self.label_size) / 2.0;
        let space_to_add = (total_width - text_width) / 2.0;

        let desired_size = vec2(total_width, line_height * 2.0 + self.font_size);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            let range = self.max - self.min;
            let v = (value - self.min) / range;
            let progress_bar = egui::ProgressBar::new(v)
                .fill(self.fg_color)
                .desired_height(self.bar_size)
                .desired_width(bar_width);

            ui.allocate_new_ui(
                egui::UiBuilder::new().max_rect(rect).layout(*ui.layout()),
                |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(label_offset);
                            ui.label(min_text);
                            ui.add(progress_bar);
                            ui.add_space(label_offset);
                            ui.label(max_text);
                        });

                        ui.horizontal(|ui| {
                            ui.add_space(space_to_add.max(0.0));
                            ui.label(text);
                        });
                    });
                },
            );
        }
        response
    }
}
