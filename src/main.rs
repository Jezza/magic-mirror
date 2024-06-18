#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::egui::{Color32, Frame, Stroke};
use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        maximized: true,
        fullscreen: true,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "magic-mirror",
        options,
        Box::new(|_ctx| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input().key_pressed(egui::Key::Escape) || ctx.input().key_pressed(egui::Key::Q) {
            frame.quit();
            return;
        }

        if ctx.input().key_pressed(egui::Key::F) {
            frame.set_fullscreen(!frame.is_fullscreen());
        }

        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::BLACK))
            .show(ctx, |ui| {
                ui.visuals_mut().widgets.noninteractive.fg_stroke.color = Color32::WHITE;

                ui.heading("Weather");
                ui.label("23°");
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::BLACK
    }
}
//
// use bevy::prelude::*;
// use bevy::window::WindowMode;
// use bevy_egui::{egui, EguiContext, EguiPlugin};
//
// fn main() {
// 	App::new()
// 		.insert_resource(ClearColor(Color::BLACK))
// 		.insert_resource(WindowDescriptor {
// 			mode: WindowMode::BorderlessFullscreen,
// 			..WindowDescriptor::default()
// 		})
// 		.add_plugins(DefaultPlugins)
// 		.add_plugin(EguiPlugin)
// 		// Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
// 		// or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
// 		.add_system(ui_example)
// 		.run();
// }
//
// fn ui_example(mut egui_context: ResMut<EguiContext>) {
//
// 	egui::Window::new("Hello")
// 		.show(egui_context.ctx_mut(), |ui| {
// 			ui.label("world");
// 		});
// }
