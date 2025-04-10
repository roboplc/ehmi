use egui::{
    epaint::PathShape, vec2, Align2, Color32, FontId, Pos2, Rect, Response, Sense, Shape, Stroke,
    Ui,
};

use crate::colors::{get_text_color, GRAY, GRAY_DARK, SUCCESS};
use core::fmt;
use std::f32::consts::PI;
use std::ops::RangeInclusive;

/// Gauge component
pub struct Gauge {
    value: f64,
    value_range: RangeInclusive<f64>,
    size: f32,
    angle_range: RangeInclusive<i16>,
    stroke_width: f32,
    text: Option<String>,
    bg_color: Option<Color32>,
    fg_color: Color32,
    text_color: Option<Color32>,
    arrow_length_factor: f32,
    arrow_width: f32,
    ticks: usize,
    tick_size: f32,
    pointer_radius: f32,
}

impl Gauge {
    /// Create a new gauge
    pub fn new<V>(value: V) -> Self
    where
        V: Into<f64>,
    {
        Self {
            value: value.into(),
            value_range: 0.0..=100.0,
            size: 200.0,
            angle_range: 0..=180,
            stroke_width: 1.5,
            text: None,
            bg_color: None,
            fg_color: SUCCESS,
            text_color: None,
            arrow_length_factor: 0.8,
            arrow_width: 3.0,
            ticks: 9,
            tick_size: 3.0,
            pointer_radius: 3.0,
        }
    }

    /// Set the range of the gauge
    pub fn range(mut self, value_range: RangeInclusive<f64>) -> Self {
        self.value_range = value_range;
        self
    }

    /// Set the size of the gauge component
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the angle range of the gauge (how long is the curve and its direction)
    pub fn angle_range(mut self, angle_range: RangeInclusive<i16>) -> Self {
        let start = angle_range.start().clamp(&-360, &360);
        let end = angle_range.end().clamp(&-360, &360);
        self.angle_range = *start..=*end;
        self
    }

    /// Set the inner text of the gauge
    pub fn text(mut self, text: impl fmt::Display) -> Self {
        self.text = Some(text.to_string());
        self
    }

    /// Set the background color of the gauge arc
    pub fn bg_color(mut self, color: Color32) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set the foreground color of the gauge arc
    pub fn fg_color(mut self, color: Color32) -> Self {
        self.fg_color = color;
        self
    }

    /// Set the color of the text
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Set the number of the ticks, number below 2 disables the ticks
    pub fn ticks(mut self, n: usize) -> Self {
        self.ticks = n;
        self
    }

    /// Set the pointer radius
    pub fn pointer_radius(mut self, size: f32) -> Self {
        self.pointer_radius = size.max(1.0);
        self
    }

    fn gauge_width(&self) -> f32 {
        self.size - self.text_clearance() * 2.0
    }

    /// Set the stroke width of the gauge arc
    pub fn stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    fn text_clearance(&self) -> f32 {
        self.size / 10.0
    }

    fn radius(&self) -> f32 {
        self.gauge_width() / 2.0
    }

    #[allow(clippy::cast_possible_truncation)]
    fn value_to_angle(&self, v: f64) -> i16 {
        let max_angle = *self.angle_range.end();
        let min_angle = *self.angle_range.start();
        let angle_range = f64::from(max_angle - min_angle);
        let min_value = self.value_range.start();
        let max_value = self.value_range.end();
        let normalized = (v - min_value) / (max_value - min_value);
        (f64::from(max_angle) - (normalized * angle_range)) as i16
    }

    /// Set the arrow length factor, a factor < 0.1 disables the arrow
    pub fn arrow_length_factor(mut self, factor: f32) -> Self {
        self.arrow_length_factor = factor.clamp(0., 1.3);
        self
    }

    fn paint(&mut self, ui: &mut Ui, outer_rect: Rect, value: f64) {
        let padding = self.text_clearance();
        let rect = outer_rect.shrink(padding);

        let min_angle = *self.angle_range.start();
        let max_angle = *self.angle_range.end();
        let current_angle = self.value_to_angle(value);

        let bg_color = if let Some(c) = self.bg_color {
            c
        } else {
            ui.style().visuals.clone().extreme_bg_color
        };

        self.paint_arc(ui, rect, min_angle, max_angle, bg_color);
        self.paint_arc(ui, rect, current_angle, max_angle, self.fg_color);

        if self.arrow_length_factor < 0.1 {
            self.paint_point(ui, rect, current_angle);
            self.paint_point(ui, rect, max_angle);
        }

        if self.ticks >= 2 {
            self.paint_ticks(ui, rect);
        }

        if self.arrow_length_factor >= 0.1 {
            self.paint_arrow(ui, rect, current_angle);
        }

        if let Some(ref text) = self.text {
            self.paint_text(ui, rect, text);
        }
    }

    fn paint_arc(&self, ui: &mut Ui, rect: Rect, start_angle: i16, end_angle: i16, color: Color32) {
        if start_angle >= end_angle {
            return;
        }

        let step = ((end_angle - start_angle) / 30).max(1);
        let mut points = Vec::with_capacity(
            usize::try_from(end_angle - start_angle).unwrap() / usize::try_from(step).unwrap(),
        );

        let mut angle = start_angle;
        while angle <= end_angle {
            points.push(position_from_angle(rect, angle, self.radius()));
            angle += step;
        }

        if angle - step < end_angle {
            points.push(position_from_angle(rect, end_angle, self.radius()));
        }

        if !points.is_empty() {
            ui.painter().add(Shape::Path(PathShape {
                points,
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(self.stroke_width, color).into(),
            }));
        }
    }

    fn paint_ticks(&self, ui: &mut Ui, rect: Rect) {
        let text_color = self.text_color.unwrap_or_else(|| {
            if ui.visuals().dark_mode {
                GRAY
            } else {
                Color32::GRAY
            }
        });

        let value_range = *self.value_range.end() - *self.value_range.start();
        #[allow(clippy::cast_precision_loss)]
        let step = value_range / (self.ticks - 1) as f64;
        let font_size = self.gauge_width() / 15.0;

        for i in 0..self.ticks {
            if i == self.ticks - 1 && self.angle_range.end() - self.angle_range.start() >= 360 {
                continue;
            }
            #[allow(clippy::cast_precision_loss)]
            let tick_value = *self.value_range.start() + step * i as f64;
            let angle = self.value_to_angle(tick_value);

            if self.ticks >= 2 {
                let tick_inner = position_from_angle(rect, angle, self.radius() - self.tick_size);
                let tick_outer = position_from_angle(rect, angle, self.radius() + self.tick_size);
                ui.painter()
                    .line_segment([tick_inner, tick_outer], Stroke::new(1.0, text_color));
            }

            let text_pos =
                position_from_angle(rect, angle, self.radius() + self.gauge_width() * 0.1);
            ui.painter().text(
                text_pos,
                Align2::CENTER_CENTER,
                format!("{}", tick_value.round()),
                FontId::proportional(font_size),
                text_color,
            );
        }
    }

    fn paint_arrow(&self, ui: &mut Ui, rect: Rect, angle: i16) {
        let center = rect.center();
        let arrow_color = GRAY_DARK;

        let arrow_length = self.radius() * self.arrow_length_factor;
        let arrow_end = position_from_angle(rect, angle, arrow_length);

        ui.painter().line_segment(
            [center, arrow_end],
            Stroke::new(self.arrow_width, arrow_color),
        );

        ui.painter()
            .circle(center, self.pointer_radius * 0.8, arrow_color, Stroke::NONE);
    }

    fn paint_text(&self, ui: &mut Ui, rect: Rect, text: &str) {
        let text_color = self.text_color.unwrap_or_else(|| get_text_color(ui));

        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            text,
            FontId::proportional(self.gauge_width() / 9.0),
            text_color,
        );
    }

    fn paint_point(&self, ui: &mut Ui, rect: Rect, angle: i16) {
        let point = position_from_angle(rect, angle, self.radius() - self.stroke_width / 2.0);
        ui.painter().circle(
            point,
            self.pointer_radius,
            self.fg_color,
            Stroke::new(1.0, self.fg_color),
        );
    }
}

impl egui::Widget for Gauge {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = vec2(self.size, self.size);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        let value = self
            .value
            .clamp(*self.value_range.start(), *self.value_range.end());

        response.widget_info(|| {
            egui::WidgetInfo::slider(true, value, self.text.as_ref().map_or("", |s| s.as_str()))
        });

        if ui.is_rect_visible(rect) {
            self.paint(ui, rect, value);
        }

        response
    }
}

fn position_from_angle(rect: Rect, angle: i16, radius: f32) -> Pos2 {
    let center = rect.center();
    let angle_rad = f32::from(angle) * PI / 180.0;
    center + vec2(angle_rad.cos() * radius, -angle_rad.sin() * radius)
}
