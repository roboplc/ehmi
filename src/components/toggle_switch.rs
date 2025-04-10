use core::fmt;

use egui::{pos2, vec2, Align2, Color32, FontId, Stroke, StrokeKind, Ui, Vec2};

use crate::colors::{get_text_color, GRAY, SUCCESS, WARN};

/// Toggle switch style
#[derive(Clone, Copy, Debug)]
pub enum ToggleStyle {
    /// Slider button
    Button,
    /// Relay switch
    Relay,
    /// Valve switch
    Valve,
}

/// Toggle switch component
pub struct ToggleSwitch<'a> {
    on: &'a mut bool,
    label: Option<String>,
    color: Color32,
    style: ToggleStyle,
    size: Option<Vec2>,
    font_size: f32,
}

impl<'a> ToggleSwitch<'a> {
    /// Create a new toggle switch
    pub fn new(on: &'a mut bool) -> Self {
        Self {
            on,
            label: None,
            color: Color32::WHITE,
            style: ToggleStyle::Button,
            size: None,
            font_size: 14.0,
        }
    }

    /// Set the label of the toggle switch
    pub fn label(mut self, label: impl fmt::Display) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Set the color of the toggle switch
    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set the style of the toggle switch
    pub fn style(mut self, style: ToggleStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the size of the toggle switch
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the size of the toggle switch label
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
}

impl egui::Widget for ToggleSwitch<'_> {
    #[allow(clippy::too_many_lines)]
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let has_user_defined_size = self.size.is_some();

        let default_toggle_size = match self.style {
            ToggleStyle::Button => vec2(2.0, 1.0),
            ToggleStyle::Relay => vec2(3.0, 1.5),
            ToggleStyle::Valve => vec2(3.0, 3.0),
        } * ui.spacing().interact_size.y;

        let (label_size, label_spacing) = if let Some(ref label_text) = self.label {
            let text_size = ui.fonts(|f| {
                let galley = f.layout_no_wrap(
                    label_text.to_string(),
                    FontId::proportional(self.font_size),
                    ui.visuals().text_color(),
                );
                galley.size()
            });
            (Some(text_size), ui.spacing().item_spacing.x)
        } else {
            (None, 0.0)
        };

        let full_size = if has_user_defined_size {
            self.size.unwrap()
        } else if let Some(text_size) = label_size {
            vec2(
                default_toggle_size.x + label_spacing + text_size.x,
                default_toggle_size.y.max(text_size.y),
            )
        } else {
            default_toggle_size
        };

        let (rect, mut response) = ui.allocate_exact_size(full_size, egui::Sense::click());

        if response.clicked() {
            *self.on = !*self.on;
            response.mark_changed();
        }

        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Checkbox,
                ui.is_enabled(),
                *self.on,
                self.label.as_ref().map_or("", |s| s.as_str()),
            )
        });

        if ui.is_rect_visible(rect) {
            let stroke_color = if *self.on { SUCCESS } else { WARN };
            let stroke = Stroke::new(1.0, stroke_color);
            let corner_radius = 4.0;
            let painter = ui.painter();
            if matches!(self.style, ToggleStyle::Relay) {
                painter.rect_stroke(rect, corner_radius, stroke, StrokeKind::Inside);
            }

            let toggle_size = if has_user_defined_size && label_size.is_some() {
                let label_text_size = label_size.unwrap();
                let available_width = full_size.x - label_spacing - label_text_size.x;
                let toggle_width = available_width.max(default_toggle_size.x * 0.6);
                vec2(toggle_width, full_size.y)
            } else if has_user_defined_size {
                full_size
            } else {
                default_toggle_size
            };

            let rect_with_margin = egui::Rect::from_min_size(rect.min, toggle_size);
            let inner_margin = if matches!(self.style, ToggleStyle::Button) {
                0.0
            } else {
                ui.spacing().item_spacing.x
            };

            if matches!(self.style, ToggleStyle::Valve) {
                painter.rect_stroke(rect_with_margin, corner_radius, stroke, StrokeKind::Inside);
            }

            let toggle_rect = rect_with_margin.shrink(inner_margin);

            let how_on = ui.ctx().animate_bool_responsive(response.id, *self.on);

            match self.style {
                ToggleStyle::Button => {
                    let radius = 0.5 * toggle_rect.height();
                    painter.rect(
                        toggle_rect,
                        radius,
                        if *self.on { SUCCESS } else { GRAY },
                        Stroke::NONE,
                        StrokeKind::Inside,
                    );
                    let circle_x = egui::lerp(
                        (toggle_rect.left() + radius)..=(toggle_rect.right() - radius),
                        how_on,
                    );
                    let center = pos2(circle_x, toggle_rect.center().y);
                    painter.circle(center, 0.75 * radius, Color32::WHITE, Stroke::NONE);
                }
                ToggleStyle::Relay => {
                    let center_y = toggle_rect.center().y;
                    let circle_radius = toggle_rect.width() * 0.04;
                    let node_left = toggle_rect.left() + toggle_rect.width() / 3.0;
                    let node_right = toggle_rect.right() - toggle_rect.width() / 3.0;

                    painter.line_segment(
                        [
                            pos2(toggle_rect.left(), center_y),
                            pos2(node_left - circle_radius * 2.0, center_y),
                        ],
                        stroke,
                    );

                    let circle_left = pos2(node_left - circle_radius, center_y);
                    let circle_right = pos2(node_right + circle_radius, center_y);

                    painter.circle_stroke(circle_left, circle_radius, stroke);
                    painter.circle_stroke(circle_right, circle_radius, stroke);

                    painter.line_segment(
                        [
                            pos2(toggle_rect.right(), center_y),
                            pos2(node_right + circle_radius * 2.0, center_y),
                        ],
                        stroke,
                    );

                    let switch_left = pos2(node_left, center_y);
                    let switch_right = pos2(node_right, center_y);
                    let off_angle = 45.0_f32.to_radians();
                    let t = ui.ctx().animate_bool(response.id, *self.on);
                    let angle = egui::lerp(off_angle..=0.0, t);
                    let length = (switch_right - switch_left).length();
                    let dir = vec2(angle.cos(), angle.sin());
                    let animated_left = switch_right - dir * length;
                    painter.line_segment([animated_left, switch_right], stroke);
                }
                ToggleStyle::Valve => {
                    let center = toggle_rect.center();
                    let radius = toggle_rect.width().min(toggle_rect.height()) * 0.2;
                    let valve_width = toggle_rect.width().min(toggle_rect.height());
                    let valve_height = valve_width;

                    let top_y = center.y - radius * 2.0;
                    painter.line_segment(
                        [
                            pos2(center.x - radius, top_y),
                            pos2(center.x + radius, top_y),
                        ],
                        stroke,
                    );

                    painter.line_segment(
                        [pos2(center.x, top_y), pos2(center.x, top_y + radius)],
                        stroke,
                    );

                    painter.add(egui::Shape::line(
                        vec![
                            pos2(center.x - valve_width / 2.0, center.y - valve_height / 4.0),
                            pos2(center.x - valve_width / 2.0, center.y + valve_height / 4.0),
                            pos2(center.x + valve_width / 2.0, center.y - valve_height / 4.0),
                            pos2(center.x + valve_width / 2.0, center.y + valve_height / 4.0),
                            pos2(center.x - valve_width / 2.0, center.y - valve_height / 4.0),
                        ],
                        stroke,
                    ));

                    let t = ui.ctx().animate_bool_with_time(response.id, *self.on, 3.0);

                    if t > 0.0 && t < 1.0 {
                        painter.circle(center, radius, ui.visuals().panel_fill, Stroke::NONE);
                        let size = vec2(radius * 2.0, radius * 2.0);
                        let spinner_rect = egui::Rect::from_center_size(center, size);
                        let spinner = egui::Spinner::new()
                            .size(radius * 2.0)
                            .color(ui.visuals().panel_fill);
                        spinner.paint_at(ui, spinner_rect);
                    } else {
                        painter.circle(center, radius, ui.visuals().panel_fill, stroke);
                    }

                    if !*self.on {
                        painter.line_segment(
                            [
                                pos2(center.x - valve_width / 2.0, center.y + valve_height / 4.0),
                                pos2(center.x + valve_width / 2.0, center.y - valve_height / 4.0),
                            ],
                            stroke,
                        );
                    }
                }
            }

            if let Some(label) = self.label {
                let label_pos = if matches!(self.style, ToggleStyle::Valve) {
                    pos2(rect.right(), rect.center().y)
                } else {
                    pos2(
                        toggle_rect.right() + ui.spacing().item_spacing.x,
                        toggle_rect.center().y,
                    )
                };
                let anchor = if matches!(self.style, ToggleStyle::Valve) {
                    Align2::RIGHT_CENTER
                } else {
                    Align2::LEFT_CENTER
                };
                painter.text(
                    label_pos,
                    anchor,
                    label,
                    FontId::proportional(self.font_size),
                    get_text_color(ui),
                );
            }
        }

        response
    }
}
