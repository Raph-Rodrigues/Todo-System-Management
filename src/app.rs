use eframe::egui;
use std::collections::HashMap;
use crate::models::TodoItem;

pub struct App {
    pub items: HashMap<u32, TodoItem>,
    pub next_id: u32,
    pub currently_edited: Option<(u32, TodoItem)>,
}

impl App {
    pub fn name() -> &'static str {
        "Todo List Manager"
    }

    pub fn add_item(&mut self) {
        self.items.insert(self.next_id, TodoItem::default());
        self.next_id += 1;
    }

    pub fn remove_item(&mut self, id: u32) {
        self.items.remove(&id);
    }

    pub fn save_item(&mut self, id: u32, item: TodoItem) {
        self.items.insert(id, item);
    }

    pub fn toggle_completion(&mut self, id: u32) {
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

            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    ui.indent("tarefa", |ui| {
                        // Criar clone seguro para iteração
                        let items = self.items.clone();

                        for (id, mut item) in items {
                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                // Checkbox de status
                                if ui.checkbox(&mut item.completed, "").clicked() {
                                    self.toggle_completion(id);
                                }

                                // Label clicável
                                let label_response = ui.add(
                                    egui::Label::new(&item.name)
                                        .sense(egui::Sense::click())
                                );

                                // Verificação de clique
                                if label_response.clicked() {
                                    self.currently_edited = Some((id, item.clone()));
                                }

                                ui.add_space(5.0);
                            });
                        }
                    });
                });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Adicionar Item").clicked() {
                    self.add_item();
                }

                if ui.button("Sair").clicked() {
                    std::process::exit(0);
                }
            });

            // Janela de edição
            if let Some((id, mut item)) = self.currently_edited.take() {
                let viewport_id = egui::ViewportId::from_hash_of(format!("edit_{}", id));
                let viewport_builder = egui::ViewportBuilder::default()
                    .with_inner_size((300.0, 300.0))
                    .with_title(format!("Editando: {}", item.name));

                let mut should_close = false;
                let mut should_save = false;
                let mut should_remove = false;

                ctx.show_viewport_immediate(viewport_id, viewport_builder, |ctx, _| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Nome:");
                        ui.text_edit_singleline(&mut item.name);
                        ui.label("Descrição:");
                        ui.text_edit_multiline(&mut item.description);
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

                match (should_save, should_remove, should_close) {
                    (true, _, _) => self.save_item(id, item),
                    (_, true, _) => self.remove_item(id),
                    (_, _, false) => self.currently_edited = Some((id, item)),
                    _ => {}
                }
            }
        });
    }
}