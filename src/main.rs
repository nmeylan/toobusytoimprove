mod web;

use std::mem;
use eframe::{App, Frame, Renderer};
use eframe::epaint::Color32;
use eframe::Theme::Light;
use egui::{ComboBox, Context, DragValue, Id, Label, RichText, Stroke, Ui, Vec2};
use egui_plot::{AxisHints, Corner, Legend, Line, LineStyle, Plot, PlotMemory, PlotPoint, PlotPoints, Points};


const BACKGROUND: Color32 = Color32::from_rgb(106, 49, 252);
const BEFORE_COLOR: Color32 = Color32::from_rgb(255, 173, 0);
const AFTER_COLOR: Color32 = Color32::from_rgb(75, 181, 67);
const INVEST_COLOR: Color32 = Color32::from_rgb(6, 122, 199);

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let options = eframe::NativeOptions {
            default_theme: Light,
            persist_window: false,
            renderer: Renderer::Glow,
            viewport: eframe::egui::ViewportBuilder::default().with_inner_size(Vec2 { x: 1200.0, y: 900.0 }).with_maximized(true),
            // viewport: egui::ViewportBuilder::default().with_inner_size(Vec2 { x: 1900.0, y: 1200.0 }).with_maximized(true),
            ..eframe::NativeOptions::default()
        };
        eframe::run_native("Too busy to improve?", options, Box::new(|_cc| {
            Ok(Box::new(MyApp::new()))
        })).unwrap();
    }
}

#[derive(Eq, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl TimeUnit {
    pub fn plural(&self) -> &str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Weeks => "weeks",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
    pub fn singular(&self) -> &str {
        match self {
            TimeUnit::Seconds => "second",
            TimeUnit::Minutes => "minute",
            TimeUnit::Hours => "hour",
            TimeUnit::Days => "day",
            TimeUnit::Weeks => "week",
            TimeUnit::Months => "month",
            TimeUnit::Years => "year",
        }
    }
    pub fn to_seconds(&self, value: f64) -> f64 {
        match self {
            TimeUnit::Seconds => value,
            TimeUnit::Minutes => value * 60.0,
            TimeUnit::Hours => value * 60.0 * 60.0,
            TimeUnit::Days => value * 60.0 * 60.0 * 24.0,
            TimeUnit::Weeks => value * 60.0 * 60.0 * 24.0 * 7.0,
            TimeUnit::Months => value * 60.0 * 60.0 * 24.0 * 30.0,
            TimeUnit::Years => value * 60.0 * 60.0 * 24.0 * 365.0,
        }
    }

    pub fn to_hours(&self, value: f64, conf_time_unit: &ConfTimeUnit) -> f64 {
        match self {
            TimeUnit::Seconds => value / 60.0 / 60.0,
            TimeUnit::Minutes => value / 60.0,
            TimeUnit::Hours => value,
            TimeUnit::Days => value * conf_time_unit.number_of_hours_per_day as f64,
            TimeUnit::Weeks => value * conf_time_unit.number_of_hours_per_day as f64 * conf_time_unit.number_of_day_per_week as f64,
            TimeUnit::Months => value * conf_time_unit.number_of_hours_per_day as f64 * conf_time_unit.number_of_day_per_month as f64,
            TimeUnit::Years => value * conf_time_unit.number_of_hours_per_day as f64 * conf_time_unit.number_of_day_per_month as f64 * 12.0,
        }
    }

    pub fn to_times_per_days(&self, value: f64, conf_time_unit: &ConfTimeUnit) -> f64 {
        match self {
            TimeUnit::Seconds => value * 60.0 * 60.0 * conf_time_unit.number_of_hours_per_day as f64,
            TimeUnit::Minutes => value * 60.0 * conf_time_unit.number_of_hours_per_day as f64,
            TimeUnit::Hours => value * conf_time_unit.number_of_hours_per_day as f64,
            TimeUnit::Days => value,
            TimeUnit::Weeks => value / conf_time_unit.number_of_day_per_week as f64,
            TimeUnit::Months => value / (conf_time_unit.number_of_day_per_month as f64),
            TimeUnit::Years => value / (conf_time_unit.number_of_day_per_month as f64 * 12.0),
        }
    }
}

struct ConfTimeUnit {
    number_of_hours_per_day: u8,
    number_of_day_per_week: u8,
    number_of_day_per_month: u8,
}

struct MyApp {
    before_taken_time: f64,
    before_taken_time_unit: TimeUnit,
    after_taken_time: f64,
    after_taken_time_unit: TimeUnit,
    invest_taken_time: f64,
    invest_taken_time_unit: TimeUnit,
    repeat_count: usize,
    repeat_count_time_unit: TimeUnit,
    scale_number_of_day: usize,
    conf_time_unit: ConfTimeUnit,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            before_taken_time: 40.0,
            before_taken_time_unit: TimeUnit::Seconds,
            after_taken_time: 10.0,
            after_taken_time_unit: TimeUnit::Seconds,
            invest_taken_time: 2.0,
            invest_taken_time_unit: TimeUnit::Hours,
            repeat_count: 20,
            repeat_count_time_unit: TimeUnit::Hours,
            scale_number_of_day: 90,

            conf_time_unit: ConfTimeUnit {
                number_of_hours_per_day: 8,
                number_of_day_per_week: 5,
                number_of_day_per_month: 22,
            },
        }
    }

    fn before_line(&self) -> Line {
        Line::new(PlotPoints::from_parametric_callback(|t| self.time_taken_per_day_in_hours(t, &self.before_taken_time_unit, self.before_taken_time),
                                                       0.0..=(self.scale_number_of_day as f64),
                                                       self.scale_number_of_day,
        ))
            .color(BEFORE_COLOR)
            .style(LineStyle::Solid)
            .name("before")
    }
    fn invest_time_line(&self, invest_time_in_hours: f64, days_needed: f64) -> Line {
        Line::new(PlotPoints::from_parametric_callback(|t| {
            let hours_per_day = self.conf_time_unit.number_of_hours_per_day as f64;
            let x = invest_time_in_hours / (hours_per_day * t);
            let res = if x < 1.0 {
                let hours_previous_days = (t - 1.0) * hours_per_day;
                hours_previous_days + invest_time_in_hours - hours_previous_days
            } else {
                t * hours_per_day
            };
            (t, res)
        },
                                                       0.0..=(days_needed),
                                                       2.max(days_needed as usize),
        ))
            .color(INVEST_COLOR)
            .style(LineStyle::Solid)
            .name("invested time")
    }
    fn after_line(&self, invest_time_in_hours: f64, after_start_at_day: f64) -> Line {
        let o = self.time_taken_per_day_in_hours(after_start_at_day, &self.after_taken_time_unit, self.after_taken_time).1;
        Line::new(PlotPoints::from_parametric_callback(|t| {
            let res = self.time_taken_per_day_in_hours(t, &self.after_taken_time_unit, self.after_taken_time);
            (t, invest_time_in_hours + res.1 - o)
        },
                                                       (after_start_at_day)..=(self.scale_number_of_day as f64),
                                                       self.scale_number_of_day,
        ))
            .color(AFTER_COLOR)
            .style(LineStyle::Solid)
            .name("after")
    }

    fn time_taken_per_day_in_hours(&self, t: f64, time_unit: &TimeUnit, time_taken: f64) -> (f64, f64) {
        let input = time_unit.to_hours(time_taken, &self.conf_time_unit);
        (t, 24.0_f64.min(input * self.repeat_count_time_unit.to_times_per_days(self.repeat_count as f64, &self.conf_time_unit)) * t)
    }

    fn intersection(&self, invest_time_in_hours: f64, after_invest_time: f64) -> (f64, f64) {
        // before line: y = ax + b
        let x = 1.0;
        let b = 0.0;
        let a = self.time_taken_per_day_in_hours(x, &self.before_taken_time_unit, self.before_taken_time).1;
        // after line: y1 = a1x + b1 - o
        let o = self.time_taken_per_day_in_hours(after_invest_time, &self.after_taken_time_unit, self.after_taken_time).1;
        let b1 = invest_time_in_hours - o;
        let a1 = self.time_taken_per_day_in_hours(x, &self.after_taken_time_unit, self.after_taken_time).1;
        let x_intersection = (b1 - b) / (a - a1);
        // println!("a: {}, a1: {}, aaa: {}, b: {}, b1: {}, x_inter: {}", a, a1, o, b, b1, x_intersection);
        (x_intersection, a * x_intersection + b)
    }

    fn label_hours_to_minutes(val: f64, short: bool) -> String {
        let seconds = TimeUnit::Hours.to_seconds(val) as usize;
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;
        if remaining_seconds > 0 {
            if short {
                format!("{}m {}s", minutes, remaining_seconds)
            } else {
                format!("{} minutes and {} seconds", minutes, remaining_seconds)
            }
        } else {
            if short {
                format!("{}m", minutes)
            } else {
                format!("{} minutes", minutes)
            }
        }
    }

    fn label_hours_to_hours_minutes(val: f64, short: bool) -> String {
        let seconds = TimeUnit::Hours.to_seconds(val) as usize;
        let hours = seconds / 60 / 60;
        let minutes = seconds / 60 % 60;
        if minutes > 0 {
            if short {
                format!("{}h {}m", hours, minutes)
            } else {
                format!("{} hours and {} minutes", hours, minutes)
            }
        } else {
            if short {
                format!("{}h", hours)
            } else {
                format!("{} hours", hours)
            }
        }
    }

    fn label_hours_to_days_hours(val: f64, short: bool, conf: &ConfTimeUnit) -> String {
        let seconds = TimeUnit::Hours.to_seconds(val) as usize;
        let days = seconds / 60 / 60 / conf.number_of_hours_per_day as usize;
        let hours = seconds / 60 / 60 % conf.number_of_hours_per_day as usize;
        if short {
            if hours == 0 {
                format!("{}d", days)
            } else {
                format!("{}d {}h", days, hours)
            }
        } else {
            if hours == 0 {
                format!("{} days", days)
            } else {
                format!("{} days and {} hours", days, hours)
            }
        }
    }

    fn value_to_human_duration(val: f64, short: bool, conf: &ConfTimeUnit) -> String {
        if val < 0.016 {
            if short {
                format!("{}s", TimeUnit::Hours.to_seconds(val))
            } else {
                format!("{} seconds", TimeUnit::Hours.to_seconds(val))
            }
        } else if val < 1.0 {
            Self::label_hours_to_minutes(val, short)
        } else if val < conf.number_of_hours_per_day as f64 {
            Self::label_hours_to_hours_minutes(val, short)
        } else {
            Self::label_hours_to_days_hours(val, short, conf)
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let invest_time_in_hours = self.invest_taken_time_unit.to_hours(self.invest_taken_time, &self.conf_time_unit);
        let after_invest_time = invest_time_in_hours / self.conf_time_unit.number_of_hours_per_day as f64;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top")
                .resizable(false)
                .default_height(150.0)
                .show_inside(ui, |ui| {
                    ui.style_mut().visuals.extreme_bg_color = BACKGROUND;
                    ui.style_mut().spacing.item_spacing = Vec2 { x: 0.0, y: 0.0 };
                    ui.style_mut().visuals.selection.stroke = Stroke::new(1.5, Color32::WHITE);
                    ui.style_mut().visuals.text_cursor.stroke = Stroke::new(1.5, Color32::WHITE);
                    let (_response_repeat_count, _response_repeat_count_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label(text("I repeat "));
                        let text_edit_repeat_count = DragValue::new(&mut self.repeat_count).range(0.0..=10000.0).speed(1.0);
                        let response_repeat_count = styled_component(ui, |ui| { ui.add(text_edit_repeat_count) });
                        ui.label(text(" times per "));
                        let repeat_time_unit = ComboBox::new("repeat_time_unit", "").selected_text(self.repeat_count_time_unit.singular());
                        let response_repeat_count_unit = styled_component(ui, |ui| {
                            repeat_time_unit.show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.repeat_count_time_unit, TimeUnit::Hours, TimeUnit::Hours.singular()).changed()
                                    || ui.selectable_value(&mut self.repeat_count_time_unit, TimeUnit::Days, TimeUnit::Days.singular()).changed()
                                    || ui.selectable_value(&mut self.repeat_count_time_unit, TimeUnit::Weeks, TimeUnit::Weeks.singular()).changed()
                                    || ui.selectable_value(&mut self.repeat_count_time_unit, TimeUnit::Months, TimeUnit::Months.singular()).changed()
                            })
                        });

                        ui.label(text(" this action."));

                        (response_repeat_count, response_repeat_count_unit)
                    }).inner;
                    ui.add_space(5.0);
                    let (_response_before_time, _response_before_time_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label(text_with_color("It takes ", BEFORE_COLOR));

                        let text_edit_before_time = DragValue::new(&mut self.before_taken_time).range(0.0..=10000.0).speed(1.0);
                        let response_before_time = styled_component(ui, |ui| { ui.add(text_edit_before_time) });
                        ui.add_space(5.0);
                        let before_time_unit = ComboBox::new("before_time_unit", "").selected_text(self.before_taken_time_unit.plural());
                        let response_before_time_unit = styled_component(ui, |ui| {
                            before_time_unit.show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.before_taken_time_unit, TimeUnit::Seconds, TimeUnit::Seconds.plural()).changed()
                                    || ui.selectable_value(&mut self.before_taken_time_unit, TimeUnit::Minutes, TimeUnit::Minutes.plural()).changed()
                                    || ui.selectable_value(&mut self.before_taken_time_unit, TimeUnit::Hours, TimeUnit::Hours.plural()).changed()
                                    || ui.selectable_value(&mut self.before_taken_time_unit, TimeUnit::Days, TimeUnit::Days.plural()).changed()
                            })
                        });
                        ui.label(text(" to complete."));
                        (response_before_time, response_before_time_unit)
                    }).inner;
                    ui.add_space(5.0);
                    let (_response_after_time, _response_after_time_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label(text_with_color("Optimizing/fixing", AFTER_COLOR));
                        ui.label(text(" the process would reduce this time to "));

                        let text_edit_after_time = DragValue::new(&mut self.after_taken_time).range(0.0..=10000.0).speed(1.0);
                        let response_after_time = styled_component(ui, |ui| { ui.add(text_edit_after_time) });
                        ui.add_space(5.0);
                        let repeat_time_unit = ComboBox::new("after_time_unit", "").selected_text(self.after_taken_time_unit.plural());
                        let response_after_time_unit = styled_component(ui, |ui| {
                            repeat_time_unit.show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.after_taken_time_unit, TimeUnit::Seconds, TimeUnit::Seconds.plural()).changed()
                                    || ui.selectable_value(&mut self.after_taken_time_unit, TimeUnit::Minutes, TimeUnit::Minutes.plural()).changed()
                                    || ui.selectable_value(&mut self.after_taken_time_unit, TimeUnit::Hours, TimeUnit::Hours.plural()).changed()
                            })
                        });
                        (response_after_time, response_after_time_unit)
                    }).inner;
                    ui.add_space(5.0);
                    let (_response_invest_time, _response_invest_time_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label(text("For this I have to "));
                        ui.label(text_with_color("invest ", INVEST_COLOR));
                        let response_invest_time = styled_component(ui, |ui| { ui.add(DragValue::new(&mut self.invest_taken_time).range(0.0..=10000.0).speed(1.0)) });
                        ui.add_space(5.0);
                        let repeat_time_unit = ComboBox::new("invest_time_unit", "").selected_text(self.invest_taken_time_unit.plural());
                        let response_invest_time_unit = crate::styled_component(ui, |ui| {
                            repeat_time_unit.show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.invest_taken_time_unit, TimeUnit::Minutes, TimeUnit::Minutes.plural()).changed()
                                    || ui.selectable_value(&mut self.invest_taken_time_unit, TimeUnit::Hours, TimeUnit::Hours.plural()).changed()
                                    || ui.selectable_value(&mut self.invest_taken_time_unit, TimeUnit::Days, TimeUnit::Days.plural()).changed()
                            })
                        });
                        (response_invest_time, response_invest_time_unit)
                    }).inner;
                    ui.add_space(10.0);
                    ui.collapsing(text("⚙ Configuration"), |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(text("I want to see a projection of next "));
                            styled_component(ui, |ui| { ui.add(DragValue::new(&mut self.scale_number_of_day).range(1.0..=10000.0).speed(2.0)) });
                            ui.label(text(" days"));
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.label(text("1 day is "));
                            styled_component(ui, |ui| { ui.add(DragValue::new(&mut self.conf_time_unit.number_of_hours_per_day).range(1.0..=24.0).speed(1.0)) });
                            ui.label(text(" hours"));
                        });
                        ui.add_space(5.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.label(text("1 week is "));
                            styled_component(ui, |ui| { ui.add(DragValue::new(&mut self.conf_time_unit.number_of_day_per_week).range(1.0..=7.0).speed(1.0)) });
                            ui.label(text(" days"));
                        });
                        ui.add_space(5.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.label(text("1 month is "));
                            styled_component(ui, |ui| { ui.add(DragValue::new(&mut self.conf_time_unit.number_of_day_per_month).range(1.0..=31.0).speed(1.0)) });
                            ui.label(text(" days"));
                        });
                    })
                });
            let intersection = self.intersection(invest_time_in_hours, after_invest_time);
            egui::TopBottomPanel::bottom("bottom").show_inside(ui, |ui| {
                if intersection.1 <= 0.0 || intersection.0 <= 0.0 || (self.after_taken_time == self.before_taken_time && self.after_taken_time_unit == self.before_taken_time_unit) {
                    ui.add(Label::new(RichText::heading(RichText::new("It looks like your optimisation will not be worth it, are you sure about data you enter?"))));
                } else if (self.scale_number_of_day as f64) < intersection.0 {
                    ui.horizontal_wrapped(|ui| {
                        ui.style_mut().spacing.item_spacing = Vec2 { x: 0.0, y: 0.0 };
                        ui.heading("After ");
                        ui.heading(RichText::new(format!("{} ", self.scale_number_of_day)).strong());
                        ui.heading(RichText::new("days ").strong());
                        ui.heading("you would not save time. ");
                        ui.heading("You will only start to save time after ");
                        let x = TimeUnit::Days.to_hours(intersection.0, &self.conf_time_unit);
                        let roi = Self::value_to_human_duration(x, false, &self.conf_time_unit);
                        ui.heading(RichText::new(roi).strong());
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.heading("Increase projection time frame in configuration (⚙)")
                    });
                } else {
                    let x = TimeUnit::Days.to_hours(intersection.0, &self.conf_time_unit);
                    let roi = Self::value_to_human_duration(x, false, &self.conf_time_unit);
                    let before = self.time_taken_per_day_in_hours(self.scale_number_of_day as f64, &self.before_taken_time_unit, self.before_taken_time).1;
                    let after = self.time_taken_per_day_in_hours(self.scale_number_of_day as f64, &self.after_taken_time_unit, self.after_taken_time).1;
                    let saved_hours = Self::value_to_human_duration(before - after, false, &self.conf_time_unit);
                    ui.horizontal_wrapped(|ui| {
                        ui.style_mut().spacing.item_spacing = Vec2 { x: 0.0, y: 0.0 };
                        ui.heading("After ");
                        ui.heading(RichText::new(format!("{} ", self.scale_number_of_day)).strong());
                        ui.heading(RichText::new("days ").strong());
                        ui.heading("you would save ");
                        ui.heading(RichText::new(format!("{}. ", saved_hours)).strong());
                        ui.heading("You will start to save time after ");
                        ui.heading(RichText::new(roi).strong());
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.style_mut().spacing.item_spacing = Vec2 { x: 0.0, y: 0.0 };
                        ui.heading("Too busy to improve? Congratulation, after ");
                        ui.heading(RichText::new(format!("{} ", self.scale_number_of_day)).strong());
                        ui.heading(RichText::new("days ").strong());
                        ui.heading("you will waste ");
                        ui.heading(RichText::new(format!("{}. ", saved_hours)).strong());
                    });
                }
            });
            let label_fmt = |_s: &str, val: &PlotPoint| {
                if val.y < 0.0 || val.x < 0.0 {
                    return String::new();
                }
                let label = Self::value_to_human_duration(val.y, true, &self.conf_time_unit);
                format!("Day: {}\nSpent time: {}", val.x.trunc(), label)
            };
            egui::CentralPanel::default().show_inside(ui, |ui| {
                let id = Id::new("plot");
                let plot = Plot::new("plot").id(id)
                    .custom_x_axes(vec![AxisHints::new_y().label("Day")])
                    .label_formatter(label_fmt)
                    .y_axis_formatter(|grid_mark, _range| {
                        if grid_mark.value <= 0.0 {
                            return String::new();
                        }
                        let label = if grid_mark.value < 0.016 {
                            format!("{}s", TimeUnit::Hours.to_seconds(grid_mark.value))
                        } else if grid_mark.value < 1.0 {
                            Self::label_hours_to_minutes(grid_mark.value, true)
                        } else {
                            Self::label_hours_to_hours_minutes(grid_mark.value, true)
                        };
                        format!("{}", label)
                    })
                    .legend(Legend::default().position(Corner::LeftTop))
                    .show_axes(true)
                    .show_grid(true)
                    ;

                let _response = plot.show(ui, |plot_ui| {
                    plot_ui.line(self.before_line());
                    plot_ui.line(self.invest_time_line(invest_time_in_hours, after_invest_time));
                    plot_ui.line(self.after_line(invest_time_in_hours, after_invest_time));
                    if intersection.1 > 0.0 && intersection.0 > 0.0 {
                        plot_ui.points(Points::new(PlotPoints::from([intersection.0, intersection.1])).color(Color32::RED).radius(2.0))
                    }
                });
                let mut plot_memory = PlotMemory::load(ctx, id);
                let mut plot_memory = mem::take(&mut plot_memory).unwrap();
                let mut transform = plot_memory.transform();
                let mut changed = false;
                if plot_memory.bounds().min()[0] < 0.0 {
                    let mut bounds = plot_memory.bounds().clone();
                    bounds.translate((plot_memory.bounds().min()[0].abs(), 0.0));
                    transform.set_bounds(bounds);
                    changed = true;
                }
                if plot_memory.bounds().min()[1] < 0.0 {
                    let mut bounds = plot_memory.bounds().clone();
                    bounds.translate((0.0, plot_memory.bounds().min()[1].abs()));
                    transform.set_bounds(bounds);
                    changed = true;
                }
                if changed {
                    plot_memory.set_transform(transform);
                    plot_memory.store(ctx, id);
                    ctx.request_repaint();
                }
            });
        });
    }
}
fn styled_component<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        let expansion = 0.0;
        // ui.style_mut().spacing.interact_size = Vec2{ x: 0.0, y: 30.0 };
        ui.style_mut().spacing.combo_height = 180.0;
        ui.style_mut().visuals.extreme_bg_color = BACKGROUND;
        ui.style_mut().visuals.widgets.active.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.active.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.widgets.active.expansion = expansion;
        ui.style_mut().visuals.widgets.noninteractive.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.widgets.noninteractive.expansion = expansion;
        ui.style_mut().visuals.widgets.open.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.open.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.widgets.open.expansion = expansion;
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.inactive.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.widgets.inactive.expansion = expansion;
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.hovered.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.widgets.hovered.expansion = expansion;
        ui.style_mut().visuals.selection.stroke = Stroke::new(1.5, Color32::WHITE);
        ui.style_mut().visuals.text_cursor.stroke = Stroke::new(1.5, Color32::WHITE);
        add_contents(ui)
    }).inner
}
fn text(text: &str) -> RichText {
    RichText::new(text).size(14.0).line_height(Some(18.0))
}
fn text_with_color(text: &str, color: Color32) -> RichText {
    RichText::new(text).size(14.0).color(color).line_height(Some(18.0))
}


#[cfg(test)]
mod tests {
    use crate::{ConfTimeUnit, MyApp, TimeUnit};

    #[test]
    fn conversion_to_hours() {
        let conf_time_unit = ConfTimeUnit {
            number_of_hours_per_day: 8,
            number_of_day_per_week: 5,
            number_of_day_per_month: 22,
        };
        assert_eq!(TimeUnit::Seconds.to_hours(2.0, &conf_time_unit), 0.0005555555555555556);
        assert_eq!(TimeUnit::Minutes.to_hours(2.0, &conf_time_unit), 0.03333333333333333);
        assert_eq!(TimeUnit::Hours.to_hours(2.0, &conf_time_unit), 2.0);
        assert_eq!(TimeUnit::Days.to_hours(2.0, &conf_time_unit), 16.0);
        assert_eq!(TimeUnit::Weeks.to_hours(2.0, &conf_time_unit), 80.0);
        assert_eq!(TimeUnit::Months.to_hours(2.0, &conf_time_unit), 352.0);
        assert_eq!(TimeUnit::Years.to_hours(2.0, &conf_time_unit), 4224.0);
    }

    #[test]
    fn conversion_time_per_day() {
        let conf_time_unit = ConfTimeUnit {
            number_of_hours_per_day: 8,
            number_of_day_per_week: 5,
            number_of_day_per_month: 30,
        };
        assert_eq!(TimeUnit::Seconds.to_times_per_days(10.0, &conf_time_unit), 288000.0);
        assert_eq!(TimeUnit::Minutes.to_times_per_days(10.0, &conf_time_unit), 4800.0);
        assert_eq!(TimeUnit::Hours.to_times_per_days(10.0, &conf_time_unit), 80.0);
        assert_eq!(TimeUnit::Days.to_times_per_days(10.0, &conf_time_unit), 10.0);
        assert_eq!(TimeUnit::Weeks.to_times_per_days(10.0, &conf_time_unit), 2.0);
        assert_eq!(TimeUnit::Months.to_times_per_days(10.0, &conf_time_unit), 0.3333333333333333);
        assert_eq!(TimeUnit::Years.to_times_per_days(10.0, &conf_time_unit), 0.027777777777777776);
    }

    #[test]
    fn time_taken() {
        let mut app = MyApp::new();
        // I repeat this action 10 times per day and it takes me 40 seconds each time
        app.repeat_count = 10;
        app.repeat_count_time_unit = TimeUnit::Days;
        app.before_taken_time = 40.0;
        app.before_taken_time_unit = TimeUnit::Seconds;
        assert_eq!(app.time_taken_per_day_in_hours(1.0, &app.before_taken_time_unit, app.before_taken_time).1.round(),
                   TimeUnit::Seconds.to_hours(40.0 * 10.0 /* 10 time per day */, &app.conf_time_unit).round());

        // I repeat this action 10 times per hour and it takes me 2 minutes each time
        app.repeat_count = 10;
        app.repeat_count_time_unit = TimeUnit::Hours;
        app.before_taken_time = 2.0;
        app.before_taken_time_unit = TimeUnit::Minutes;
        assert_eq!(app.time_taken_per_day_in_hours(1.0, &app.before_taken_time_unit, app.before_taken_time).1.round(),
                   TimeUnit::Minutes.to_hours(2.0 * 80.0 /*80 times per day (8 h * 10 time)*/, &app.conf_time_unit).round());

        // I repeat this action 2 times per week and it takes me 4 hours each time
        app.repeat_count = 2;
        app.repeat_count_time_unit = TimeUnit::Weeks;
        app.before_taken_time = 4.0;
        app.before_taken_time_unit = TimeUnit::Hours;
        assert_eq!(app.time_taken_per_day_in_hours(1.0, &app.before_taken_time_unit, app.before_taken_time).1.round(),
                   TimeUnit::Hours.to_hours(4.0 * 0.4 /* 0.4 times per day (2 time / 5 day)*/, &app.conf_time_unit).round());
    }
}