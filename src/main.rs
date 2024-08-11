use std::sync::Arc;
use eframe::{App, Frame, Renderer};
use eframe::Theme::Light;
use egui::{Context, Vec2};

fn main() {
    let options = eframe::NativeOptions {
        default_theme: Light,
        persist_window: false,
        renderer: Renderer::Glow,
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size(Vec2 { x: 1200.0, y: 900.0 }).with_maximized(true),
        // viewport: egui::ViewportBuilder::default().with_inner_size(Vec2 { x: 1900.0, y: 1200.0 }).with_maximized(true),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("Wasted time calculator", options, Box::new(|cc| {

        Ok(Box::new(MyApp::new()))
    })).unwrap();
}

struct MyApp {
    before_taken_time: f64,
    after_taken_time: f64,
    repeated_count_per_day: usize,
    scale_number_of_day: usize,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            before_taken_time: 0.0,
            after_taken_time: 0.0,
            repeated_count_per_day: 0,
            scale_number_of_day: 30,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("aaa");
        });
    }
}