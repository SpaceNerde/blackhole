use eframe::egui;
use egui_plot::{Legend, Line, PlotPoints};

mod signal;

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
enum State {
    #[default]
    DragAndDrop, 
    Plot
}

#[derive(Default)]
struct App {
    state: State,
    dropped_files: Vec<egui::DroppedFile>,
    path: Option<String>,
    fft: bool,
    points: Vec<Vec<[f64; 2]>>,
}

impl App {
    fn new() -> Self {
        App::default() 
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.state {
            State::DragAndDrop => {
                self.render_drag_and_drop(ctx, frame);
            },
            State::Plot => {
                self.render_plots(ctx, frame);
            },
        }
    }
}

impl App {
    fn render_drag_and_drop(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag and drop ur file containing the signal");
                
            if ui.button("Commit").clicked() {
                self.path = Some(self.dropped_files[0].path.clone().unwrap().display().to_string());
                self.points = signal::create_data_points(self.path.clone().unwrap());
                self.state = State::Plot;
            }

            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };

                        let mut additional_info = vec![];
                        if !file.mime.is_empty() {
                            additional_info.push(format!("type: {}", file.mime));
                        }
                        if let Some(bytes) = &file.bytes {
                            additional_info.push(format!("{} bytes", bytes.len()));
                        }
                        if !additional_info.is_empty() {
                            info += &format!(" ({})", additional_info.join(", "));
                        }

                        ui.label(info);
                    }
                });
            }
        });

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });
    }

    fn render_plots(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                egui_plot::Plot::new("sample_plot")
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .height(384.)
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        plot_ui.line(Line::new(PlotPoints::new(self.points[0].clone())).name("Samples"));
                });
                egui_plot::Plot::new("fft_plot")
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .height(384.)
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        plot_ui.line(Line::new(PlotPoints::new(self.points[1].clone())).name("FFT"));
                });
            });
        });
    }
}
