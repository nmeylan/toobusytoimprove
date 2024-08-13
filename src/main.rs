use eframe::{App, Frame, Renderer};
use eframe::epaint::Color32;
use eframe::Theme::Light;
use egui::{Align, ComboBox, Context, DragValue, Label, Layout, RichText, Stroke, TextEdit, TextStyle, Ui, Vec2};
use egui_plot::{AxisHints, CoordinatesFormatter, Corner, Legend, Line, LineStyle, Plot, PlotPoints};


const BACKGROUND: Color32 = Color32::from_rgb(106, 49, 252);

fn main() {
    let options = eframe::NativeOptions {
        default_theme: Light,
        persist_window: false,
        renderer: Renderer::Glow,
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size(Vec2 { x: 1200.0, y: 900.0 }).with_maximized(true),
        // viewport: egui::ViewportBuilder::default().with_inner_size(Vec2 { x: 1900.0, y: 1200.0 }).with_maximized(true),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("Too busy to improve", options, Box::new(|cc| {
        Ok(Box::new(MyApp::new()))
    })).unwrap();
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

#[derive(Eq, PartialEq)]
enum GuesstimationLevel {
    Good,
    Mid,
    Bad,
    Random,
}

impl GuesstimationLevel {
    pub fn as_text(&self) -> &str {
        match self {
            GuesstimationLevel::Good => "good",
            GuesstimationLevel::Mid => "ok",
            GuesstimationLevel::Bad => "bad",
            GuesstimationLevel::Random => "random"
        }
    }
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
            TimeUnit::Days => value * 60.0 * 60.0 * 24.0 ,
            TimeUnit::Weeks => value * 60.0 * 60.0 * 24.0  * 7.0 ,
            TimeUnit::Months => value * 60.0 * 60.0 * 24.0  * 30.0 ,
            TimeUnit::Years => value * 60.0 * 60.0 * 24.0  * 365.0,
        }
    }

    pub fn to_hours(&self, value: f64) -> f64 {
        match self {
            TimeUnit::Seconds => value / 60.0 / 60.0,
            TimeUnit::Minutes => value / 60.0,
            TimeUnit::Hours => value,
            TimeUnit::Days => value * 24.0,
            TimeUnit::Weeks => value * 24.0 * 7.0 ,
            TimeUnit::Months => value * 24.0 * 30.0 ,
            TimeUnit::Years => value * 24.0 * 365.0,
        }
    }

    pub fn to_times_per_days(&self, value: f64, conf_time_unit: &ConfTimeUnit) -> f64 {
        match self {
            TimeUnit::Seconds => value * 60.0 * 60.0 * conf_time_unit.number_of_hours_per_day as f64,
            TimeUnit::Minutes => value * 60.0 * conf_time_unit.number_of_hours_per_day as f64,
            TimeUnit::Hours => value * conf_time_unit.number_of_hours_per_day as f64,
            TimeUnit::Days => value,
            TimeUnit::Weeks => value / conf_time_unit.number_of_day_per_week as f64,
            TimeUnit::Months => value / (conf_time_unit.number_of_hours_per_day as f64 * conf_time_unit.number_of_day_per_month as f64),
            TimeUnit::Years => value / (conf_time_unit.number_of_hours_per_day as f64 * conf_time_unit.number_of_day_per_month as f64 * 365.0),
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
    y_axis_time_unit: TimeUnit,
    scale_number_of_day: usize,
    conf_time_unit: ConfTimeUnit,
    guesstimation_level: GuesstimationLevel,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            before_taken_time: 2.0,
            before_taken_time_unit: TimeUnit::Seconds,
            after_taken_time: 0.0,
            after_taken_time_unit: TimeUnit::Seconds,
            invest_taken_time: 0.0,
            invest_taken_time_unit: TimeUnit::Minutes,
            repeat_count: 10,
            repeat_count_time_unit: TimeUnit::Days,
            y_axis_time_unit: TimeUnit::Hours,
            scale_number_of_day: 30,

            conf_time_unit: ConfTimeUnit {
                number_of_hours_per_day: 8,
                number_of_day_per_week: 5,
                number_of_day_per_month: 22,
            },
            guesstimation_level: GuesstimationLevel::Mid,
        }
    }

    fn before_line(&self) -> Line {
        let before_taken_time = 24.0_f64.max(self.before_taken_time_unit.to_hours(self.before_taken_time));
        Line::new(PlotPoints::from_parametric_callback(
            move |t| (t, before_taken_time * self.repeat_count_time_unit.to_times_per_days(self.repeat_count as f64, &self.conf_time_unit) * t),
            0.0..=(self.scale_number_of_day as f64),
            self.scale_number_of_day,
        ))
            .color(Color32::from_rgb(100, 150, 250))
            .style(LineStyle::Solid)
            .name("before")
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top")
                .resizable(false)
                .default_height(150.0)
                .show_inside(ui, |ui| {
                    let text_color = ui.style().visuals.text_color();
                    ui.style_mut().visuals.extreme_bg_color = BACKGROUND;
                    ui.style_mut().spacing.item_spacing = Vec2 { x: 0.0, y: 0.0 };
                    ui.style_mut().visuals.selection.stroke = Stroke::new(1.5, Color32::WHITE);
                    ui.style_mut().visuals.text_cursor.stroke = Stroke::new(1.5, Color32::WHITE);
                    let (response_repeat_count, response_repeat_count_unit) = ui.horizontal_wrapped(|ui| {
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
                    let (response_before_time, response_before_time_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label(text("Which takes "));

                        let text_edit_before_time = DragValue::new(&mut self.before_taken_time).range(0.0..=360.0).speed(1.0);
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
                    let (response_after_time, response_after_time_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label(text("Optimizing/fixing the process would reduce this time to "));

                        let text_edit_after_time = DragValue::new(&mut self.after_taken_time).range(0.0..=360.0).speed(1.0);
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
                    let (response_invest_time, response_invest_time_unit, response_guestimation) = ui.horizontal_wrapped(|ui| {
                        ui.label(text("For this I have to invest "));
                        let mut text_edit_invest_time = DragValue::new(&mut self.invest_taken_time).range(0.0..=360.0).speed(1.0);
                        let response_invest_time = styled_component(ui, |ui| { ui.add(text_edit_invest_time) });
                        ui.add_space(5.0);
                        let repeat_time_unit = ComboBox::new("invest_time_unit", "").selected_text(self.invest_taken_time_unit.plural());
                        let response_invest_time_unit = crate::styled_component(ui, |ui| {
                            repeat_time_unit.show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.invest_taken_time_unit, TimeUnit::Minutes, TimeUnit::Minutes.plural()).changed()
                                    || ui.selectable_value(&mut self.invest_taken_time_unit, TimeUnit::Hours, TimeUnit::Hours.plural()).changed()
                                    || ui.selectable_value(&mut self.invest_taken_time_unit, TimeUnit::Days, TimeUnit::Days.plural()).changed()
                            })
                        });
                        ui.label(text(". Trust me I am "));
                        let repeat_time_unit = ComboBox::new("guesstimation_level", "").selected_text(self.guesstimation_level.as_text());
                        let response_guestimation = crate::styled_component(ui, |ui| {
                            repeat_time_unit.show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.guesstimation_level, GuesstimationLevel::Good, GuesstimationLevel::Good.as_text()).changed()
                                    || ui.selectable_value(&mut self.guesstimation_level, GuesstimationLevel::Mid, GuesstimationLevel::Mid.as_text()).changed()
                                    || ui.selectable_value(&mut self.guesstimation_level, GuesstimationLevel::Bad, GuesstimationLevel::Bad.as_text()).changed()
                                    || ui.selectable_value(&mut self.guesstimation_level, GuesstimationLevel::Random, GuesstimationLevel::Random.as_text()).changed()
                            })
                        });
                        ui.label(text(" at guesstimation"));
                        (response_invest_time, response_invest_time_unit, response_guestimation)
                    }).inner;
                });

            egui::TopBottomPanel::bottom("bottom").show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Within");
                    ui.label(RichText::new("90").strong()); // TODO time scale selectedd
                    ui.label(RichText::new("days").strong()); // TODO  unit selected
                    ui.label("you would have saved");
                    ui.label(RichText::new("20").strong()); // heuristic for unit
                    ui.label(RichText::new("hours").strong()); // heuristic for unit
                    ui.label("You would have started to save time after");
                    ui.label(RichText::new("1").strong()); // heuristic for unit
                    ui.label(RichText::new("day").strong());
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Congratulation").strong());
                    ui.label(", within ");
                    ui.label(RichText::new("90").strong()); // TODO time scale selectedd
                    ui.label(RichText::new("days").strong()); // TODO  unit selected
                    ui.label("you have wasted");
                    ui.label(RichText::new("20").strong()); // heuristic for unit
                    ui.label(RichText::new("hours").strong()); // heuristic for unit
                    ui.label("because you were too busy to improve!");
                });
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                let y_axes = vec![
                    AxisHints::new_y().label(self.y_axis_time_unit.plural())];
                let mut plot = Plot::new("lines_demo")
                    .legend(Legend::default())
                    .custom_y_axes(y_axes)
                    .show_axes(true)
                    .show_grid(true);
                plot = plot.coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());

                plot.show(ui, |plot_ui| {
                    plot_ui.line(self.before_line());
                })
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