#[derive(Clone, Debug, PartialEq)]
pub struct TodoItem {
    pub name: String,
    pub description: String,  // Campo renomeado de 'body' para 'description'
    pub completed: bool,
}

impl Default for TodoItem {
    fn default() -> Self {
        Self {
            name: "Nova Tarefa".to_owned(),
            description: String::new(),
            completed: false,
        }
    }
}
