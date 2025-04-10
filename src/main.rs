use eframe::egui;

#[derive(Default)]
struct App {
   // TODO: Adicionar os campos necessários para o sistema de gerenciamento de tarefas 
}

impl App {
    fn name() -> &'static str {
        "Todo System Management"
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gerenciamento de Tarefas");

            ui.separator();

            ui.label("Suas tarefas:");

            ui.separator();

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..eframe::NativeOptions::default()
    };
   
   eframe::run_native(
        App::name(),
        native_options,
        Box::new(|_| Ok(Box::<App>::default())),
   )
}