mod models;
mod app;

use eframe;
use app::App;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size((600.0, 600.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(App::name(),
                        native_options, 
                        Box::new(|_| Ok(Box::new(App::default()))))
}