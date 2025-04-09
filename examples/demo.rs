use std::time::Duration;

use atomic_timer::AtomicTimer;
use egui::{vec2, CentralPanel, Color32, Slider, Visuals};
use ehmi::{Bar, Gauge, ToggleStyle, ToggleSwitch};

const DANGER: Color32 = Color32::RED;

struct MyApp {
    value: Value,
    toggle1: bool,
    dark_mode: bool,
    pixels_per_point: f32,
}

struct Value {
    v: f32,
    dir: f32,
    timer: AtomicTimer,
}

impl Value {
    fn tick(&mut self) {
        if !self.timer.reset_if_expired() {
            return;
        }
        self.v += self.dir;
        if self.v < 0.0 {
            self.v = 0.0;
            self.dir = 0.5;
        } else if self.v > 100.0 {
            self.v = 100.0;
            self.dir = -0.5;
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self {
            v: 0.0,
            dir: 1.0,
            timer: AtomicTimer::new(Duration::from_millis(20)),
        }
    }
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            value: Value::default(),
            toggle1: true,
            dark_mode: true,
            pixels_per_point: 1.0,
        }
    }
}

impl eframe::App for MyApp {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("settings").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.dark_mode, "Dark Mode").changed() {
                    let visuals = if self.dark_mode {
                        Visuals::dark()
                    } else {
                        Visuals::light()
                    };

                    ctx.set_visuals(visuals);
                }

                ui.separator();

                ui.label("Pixels per point:");
                if ui
                    .add(Slider::new(&mut self.pixels_per_point, 0.5..=3.0).step_by(0.1))
                    .changed()
                {
                    ctx.set_pixels_per_point(self.pixels_per_point);
                }

                if ui.button("Reset").clicked() {
                    self.pixels_per_point = 1.0;
                    ctx.set_pixels_per_point(1.0);
                }
            });
        });

        let value = self.value.v;

        CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().slider_width = 300.0;
            ui.add(Slider::new(&mut self.value.v, 0.0..=100.0));
            ui.horizontal(|ui| {
                ui.add(
                    Bar::new(value)
                        .text(format!("T {:>6.1}°C", value))
                        .vertical(80.)
                        .range(-20.0..=80.0),
                );
                ui.add(
                    Bar::new(value)
                        .text(format!("T {:>6.1}°C", value))
                        .vertical(80.)
                        .fg_color(DANGER)
                        .bar_size(10.0),
                );
                ui.vertical(|ui| {
                    ui.add(Bar::new(value).text("Hello"));
                    ui.add(
                        Bar::new(value)
                            .text("Hello")
                            .fg_color(DANGER)
                            .bar_size(10.0)
                            .font_size(40.0)
                            .label_size(20.0)
                            .range(-20.0..=120.0),
                    );

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            if ui
                                .add(ToggleSwitch::new(&mut self.toggle1).label("Hello"))
                                .clicked()
                            {
                                println!("New state: {}", self.toggle1);
                            }
                            ui.add(
                                ToggleSwitch::new(&mut self.toggle1)
                                    .label("Relay")
                                    .style(ToggleStyle::Relay),
                            );
                            ui.add(
                                ToggleSwitch::new(&mut self.toggle1)
                                    .label("Valve")
                                    .style(ToggleStyle::Valve),
                            );
                        });
                        ui.add(
                            ToggleSwitch::new(&mut self.toggle1)
                                .label("C")
                                .style(ToggleStyle::Relay)
                                .size(vec2(100.0, 100.0)),
                        );

                        ui.add(
                            ToggleSwitch::new(&mut self.toggle1)
                                .label("Hello")
                                .style(ToggleStyle::Valve)
                                .size(vec2(100.0, 100.0)),
                        );
                        ui.add(
                            ToggleSwitch::new(&mut self.toggle1)
                                .label("Hello")
                                .size(vec2(100.0, 30.0)),
                        );
                    })
                });
            });

            ui.horizontal(|ui| {
                ui.add(
                    Gauge::new(value)
                        .range(0.0..=100.0)
                        .size(200.0)
                        .text(format!("light {:>6.1}", value)),
                );
                ui.separator();
                ui.add(
                    Gauge::new(value)
                        .range(0.0..=150.0)
                        .text(format!("sphere {:>6.1}", value))
                        .size(200.0)
                        .ticks(5)
                        .angle_range(-90..=270),
                );
                ui.separator();
                let mut gauge = Gauge::new(value)
                    .range(10.0..=80.0)
                    .size(200.0)
                    .text(format!("modern {:>6.1}", value))
                    .ticks(0)
                    .arrow_length_factor(0.)
                    .angle_range(-45..=225);
                if !(10. ..=80.).contains(&value) {
                    gauge = gauge.fg_color(DANGER).text_color(DANGER);
                }
                ui.add(gauge)
            });
        });
        if self.toggle1 {
            self.value.tick();
        }
        ctx.request_repaint_after(Duration::from_millis(20));
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "ehmi",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
