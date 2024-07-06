use std::sync::mpsc;
use std::thread::JoinHandle;

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Blackhole",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

#[derive(Default)]
struct App {
    file: egui::DroppedFile,
    path: Option<String>,
}

impl App {
    fn new() -> Self {
        App::default() 
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag and drop ur file containing the signal");
        });
    }
}
