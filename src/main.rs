use eframe::egui;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct TodoItem {
    name: String,
    body: String,
    completed: bool,
}

impl Default for TodoItem {
    fn default() -> Self {
        Self {
            name: "New Todo Item".to_owned(),
            body: String::new(),
            completed: false,
        }
    }
}

struct App {
    items: HashMap<u32, TodoItem>,
    next_id: u32,
    currently_edited: Option<(u32, TodoItem)>,
}

impl App {
    fn name() -> &'static str {
        "Todo List Manager"
    }

    fn add_item(&mut self) {
        self.items.insert(self.next_id, TodoItem::default());
        self.next_id += 1;
    }

    fn remove_item(&mut self, id: u32) {
        self.items.remove(&id);
    }

    fn save_item(&mut self, id: u32, item: TodoItem) {
        self.items.insert(id, item);
    }

    fn toggle_completion(&mut self, id: u32) {
        if let Some(item) = self.items.get_mut(&id) {
            item.completed = !item.completed;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            next_id: 0,
            currently_edited: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lista de Tarefas");
            ui.monospace("Clique 'Adicionar Item' para adicionar itens à lista.");
            ui.monospace("Clique em um item para visualizar, editar ou remover.");
            ui.monospace("O item abrirá em uma nova janela.");
            ui.separator();

            let items = self.items.clone();

            ui.indent("tarefa", |ui| {
                for (id, mut item) in items {
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        if ui.checkbox(&mut item.completed, "").clicked() {
                            self.toggle_completion(id);
                        }

                        if ui
                            .add(egui::Label::new(&item.name).sense(egui::Sense::click()))
                            .clicked()
                        {
                            self.currently_edited = Some((id, item));
                        };
                    });

                    ui.add_space(5.0);
                }
            });

            if ui.button("Adicionar Item").clicked() {
                self.add_item();
            }

            ui.separator();

            if ui.button("Sair").clicked() {
                std::process::exit(0);
            }

            if let Some((id, mut item)) = self.currently_edited.take() {
                let viewport_id = egui::ViewportId::from_hash_of(format!("edit {id}"));
                let viewport_builder = egui::ViewportBuilder::default()
                    .with_inner_size((300.0, 300.0))
                    .with_title(format!("editar {}", item.name));

                let mut should_close = false;
                let mut should_save = false;
                let mut should_remove = false;

                ctx.show_viewport_immediate(viewport_id, viewport_builder, |ctx, _| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Nome:");
                        ui.text_edit_singleline(&mut item.name);
                        ui.label("Descrição:");
                        ui.text_edit_multiline(&mut item.body);
                        ui.checkbox(&mut item.completed, "Concluído");

                        ui.horizontal(|ui| {
                            if ui.button("Salvar").clicked() {
                                should_save = true;
                                should_close = true;
                            }
                            if ui.button("Cancelar").clicked() {
                                should_close = true;
                            }
                            if ui.button("Remover").clicked() {
                                should_remove = true;
                                should_close = true;
                            }
                        });
                    });
                });

                if should_save {
                    self.save_item(id, item);
                } else if should_remove {
                    self.remove_item(id);
                } else if !should_close {
                    self.currently_edited = Some((id, item));
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((600.0, 600.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        App::name(),
        native_options,
        Box::new(|_| Ok(Box::new(App::default()))),
    )
}