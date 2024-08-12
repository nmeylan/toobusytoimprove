use std::sync::Arc;
use eframe::{App, Frame, Renderer};
use eframe::epaint::Color32;
use eframe::Theme::Light;
use egui::{ComboBox, Context, Label, RichText, Stroke, TextEdit, Ui, Vec2, Widget};
use egui::style::TextCursorStyle;
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

enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl TimeUnit {
    pub fn display(&self) -> &str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days"
        }
    }
}

struct MyApp {
    before_taken_time: f64,
    before_taken_time_unit: TimeUnit,
    after_taken_time: f64,
    after_taken_time_unit: TimeUnit,
    invest_taken_time: f64,
    invest_taken_time_unit: TimeUnit,
    repeat_count: usize,
    y_axis_time_unit: TimeUnit,
    scale_number_of_day: usize,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            before_taken_time: 2.0,
            before_taken_time_unit: TimeUnit::Seconds,
            after_taken_time: 0.0,
            after_taken_time_unit: TimeUnit::Seconds,
            invest_taken_time: 0.0,
            invest_taken_time_unit: TimeUnit::Seconds,
            repeat_count: 10,
            y_axis_time_unit: TimeUnit::Hours,
            scale_number_of_day: 30,

        }
    }

    fn before_line(&self) -> Line {
        let before_taken_time = self.before_taken_time;
        Line::new(PlotPoints::from_parametric_callback(
            move |t| (t, before_taken_time * self.repeat_count as f64 * t),
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
                    ui.style_mut().visuals.selection.stroke = Stroke::new(1.5, Color32::WHITE);
                    ui.style_mut().visuals.text_cursor.stroke = Stroke::new(1.5, Color32::WHITE);
                    let (response_repeat_count, response_repeat_count_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label("I repeat");
                        let mut x = format!("{}", self.repeat_count);
                        let mut text_edit_repeat_count = TextEdit::singleline(&mut x).text_color(Color32::WHITE);
                        text_edit_repeat_count = text_edit_repeat_count.desired_width(30.0);
                        let response_repeat_count = ui.add(text_edit_repeat_count);
                        ui.label("times per");
                        let repeat_time_unit = ComboBox::new("repeat_time_unit", "");
                        let response_repeat_count_unit = styled_combobox(ui, |ui| {
                            repeat_time_unit.show_ui(ui, |ui| {
                                ui.label("Hour");
                                ui.label("Day");
                                ui.label("Week");
                                ui.label("Month");
                            })
                        });

                        ui.label("this action.");

                        (response_repeat_count, response_repeat_count_unit)
                    }).inner;
                    let (response_before_time, response_before_time_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Which take").color(text_color));
                        let mut x = format!("{}", self.before_taken_time);
                        let mut text_edit_before_time = TextEdit::singleline(&mut x).text_color(Color32::WHITE);
                        text_edit_before_time = text_edit_before_time.desired_width(30.0);
                        let response_before_time = ui.add(text_edit_before_time);
                        let before_time_unit = ComboBox::new("before_time_unit", "");
                        let response_before_time_unit = styled_combobox(ui, |ui| {
                            before_time_unit.show_ui(ui, |ui| {
                                ui.label("Second");
                                ui.label("Minutes");
                                ui.label("Hours");
                            })
                        });
                        ui.label("to complete.");
                        (response_before_time, response_before_time_unit)
                    }).inner;
                    let (response_after_time, response_after_time_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label("Optimizing/fixing the process would reduce this time to");

                        let mut x = format!("{}", self.after_taken_time);
                        let mut text_edit_after_time = TextEdit::singleline(&mut x).text_color(Color32::WHITE);
                        text_edit_after_time = text_edit_after_time.desired_width(30.0);
                        let response_after_time = ui.add(text_edit_after_time);
                        let repeat_time_unit = ComboBox::new("after_time_unit", "");
                        let response_after_time_unit = styled_combobox(ui, |ui| {
                            repeat_time_unit.show_ui(ui, |ui| {
                                ui.label("Second");
                                ui.label("Minutes");
                                ui.label("Hours");
                            })
                        });
                        (response_after_time, response_after_time_unit)
                    }).inner;

                    let (response_invest_time, response_invest_time_unit) = ui.horizontal_wrapped(|ui| {
                        ui.label("For this I have to invest");
                        let mut x = format!("{}", self.invest_taken_time);
                        let mut text_edit_invest_time = TextEdit::singleline(&mut x).text_color(Color32::WHITE);
                        text_edit_invest_time = text_edit_invest_time.desired_width(30.0);
                        let response_invest_time = ui.add(text_edit_invest_time);
                        let repeat_time_unit = ComboBox::new("invest_time_unit", "");
                        let response_invest_time_unit = crate::styled_combobox(ui, |ui| {
                            repeat_time_unit.show_ui(ui, |ui| {
                                ui.label("Minutes");
                                ui.label("Hours");
                                ui.label("Days");
                            })
                        });
                        (response_invest_time, response_invest_time_unit)
                    }).inner;
                });

            egui::TopBottomPanel::bottom("bottom").show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Within");
                    ui.label("90"); // TODO time scale selectedd
                    ui.label("days"); // TODO  unit selected
                    ui.label("you would have saved");
                    ui.label("20"); // heuristic for unit
                    ui.label("hours"); // heuristic for unit
                    ui.label("You would have started to save time after");
                    ui.label("1"); // heuristic for unit
                    ui.label("day");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label("Within");
                    ui.label("90"); // TODO time scale selectedd
                    ui.label("days"); // TODO  unit selected
                    ui.label("you have wasted");
                    ui.label("20"); // heuristic for unit
                    ui.label("hours"); // heuristic for unit
                    ui.label("because you were too busy to improve!");
                });
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                let y_axes = vec![
                    AxisHints::new_y().label(self.y_axis_time_unit.display())];
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
fn styled_combobox<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.style_mut().visuals.extreme_bg_color = BACKGROUND;
        ui.style_mut().visuals.widgets.active.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.active.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.widgets.noninteractive.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.widgets.open.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.open.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.inactive.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = BACKGROUND;
        ui.style_mut().visuals.widgets.hovered.fg_stroke.color = Color32::WHITE;
        ui.style_mut().visuals.selection.stroke = Stroke::new(1.5, Color32::WHITE);
        ui.style_mut().visuals.text_cursor.stroke = Stroke::new(1.5, Color32::WHITE);
        add_contents(ui)
    }).inner
}