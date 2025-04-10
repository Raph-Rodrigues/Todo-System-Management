use eframe::egui;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct TodoItem {
    name: String,
    body: String,
}

impl Default for TodoItem {
    fn default() -> Self {
        Self {
            name: "New Todo Item".to_owned(),
            body: String::new(),
        }
    }
}

struct ExampleApp {
    items: HashMap<u32, TodoItem>,
    next_id: u32,
    currently_edited: Option<(u32, TodoItem)>,
}

impl ExampleApp {
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
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            next_id: 0,
            currently_edited: None,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Todo List");
            ui.monospace("Click 'Add Item' to add items to the list.");
            ui.monospace("Click on an item to view, edit, or remove it.");
            ui.monospace("The item will open in a new window.");
            ui.separator();

            let items = self.items.clone();

            ui.indent("todo_items", |ui| {
                for (id, item) in items {
                    ui.add_space(5.0);

                    if ui
                        .add(egui::Label::new(&item.name).sense(egui::Sense::click()))
                        .clicked()
                    {
                        self.currently_edited = Some((id, item));
                    };

                    ui.add_space(5.0);
                }
            });

            if ui.button("Add Item").clicked() {
                self.add_item();
            }

            ui.separator();

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }

            if let Some((id, mut item)) = self.currently_edited.take() {
                let viewport_id = egui::ViewportId::from_hash_of(format!("edit {id}"));
                let viewport_builder = egui::ViewportBuilder::default()
                    .with_inner_size((300.0, 300.0))
                    .with_title(format!("edit {}", item.name));

                let mut should_close = false;
                let mut should_save = false;
                let mut should_remove = false;

                ctx.show_viewport_immediate(viewport_id, viewport_builder, |ctx, _| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut item.name);
                        ui.label("Body:");
                        ui.text_edit_multiline(&mut item.body);

                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                should_save = true;
                                should_close = true;
                            }
                            if ui.button("Cancel").clicked() {
                                should_close = true;
                            }
                            if ui.button("Remove").clicked() {
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
        ExampleApp::name(),
        native_options,
        Box::new(|_| Ok(Box::new(ExampleApp::default()))),
    )
}