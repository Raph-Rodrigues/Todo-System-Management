use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    title: String,
    description: String,
    completed: bool
}

impl Task {
    fn new(title: String, description: String) -> Self {
        Task { title, description, completed: false, }
    }
}

fn add_task(tasks: &mut Vec<Task>, title: String, description: String) {
    tasks.push(Task::new(title, description));
}

fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("Nao foram encontradas tarefas!");
        return;
    }

    for (index, task) in tasks.iter().enumerate() {
        println!("Tarefa {}:", index + 1);
        println!(" Titulo:    {} ", task.title);
        println!(" Descricao: {}", task.description);
        println!(" Status:    {}\n", if task.completed {"[X]"} else {"[ ]"} );
    }
}

fn remove_task(tasks: &mut Vec<Task>, index: usize) -> Result<(), &'static str> {
    if index < tasks.len() {
        tasks.remove(index);
        Ok(())
    } else {
        Err("Indice invalido!")
    }
}

fn mark_task_as_done(tasks: &mut Vec<Task>, index: usize) -> Result<(), &'static str> {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
        Ok(())
    } else {
        Err("Indice invalido")
    }
}

fn main() {
    println!("Bem vindo ao Gerenciador de Tarefas!");
    println!("-----------------------------------\n");

    let mut tasks = Vec::new();
    // Tasks de exemplo
    add_task(&mut tasks, "Estudar Rust".into(), "Aprender a linguagem Rust".into());
    add_task(&mut tasks, "Estudar Egui".into(), "Aprender o framework egui".into());

    loop {
        println!("\nOpcoes:");
        println!("1 - Adicionar Tarefa");
        println!("2 - Listar Tarefas");
        println!("3 - Remover Tarefa");
        println!("4 - Marcar Tarefa como Concluida");
        println!("5 - Sair");
        print!("Escolha uma opcao: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler a entrada");

        match input.trim().parse::<u32>() {
            Ok(1) => {
                let mut title = String::new();
                let mut description = String::new();

                print!("Digite o titulo da tarefa: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).expect("Erro ao ler o titulo");

                print!("Digite a descricao da tarefa: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).expect("Erro ao ler a descricao");

                add_task(&mut tasks, title.trim().to_string(), description.trim().to_string());
                println!("Tarefa adicionada com sucesso!");
            }
            Ok(2) => list_tasks(&tasks),
            Ok(3) => {
                print!("Digite o indice da tarefa a ser removida: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Erro ao ler o indice");

                match index.trim().parse::<usize>() {
                    Ok(n) if n > 0 && n <= tasks.len() => {
                        if let Err(e) = remove_task(&mut tasks, n - 1) {
                            print!("{}", e);
                        } else {
                            println!("Tarefa removida com sucesso!");
                        }
                    }
                    _ => println!("Indice invalido!"),
                }
            }
            Ok(4) => {
                print!("Digite o indice da tarefa a ser marcada como concluida: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Erro ao ler o indice");

                match index.trim().parse::<usize>() {
                    Ok(n) if n > 0 && n <= tasks.len() => {
                        if let Err(e) = mark_task_as_done(&mut tasks, n - 1) {
                            println!("{}", e);
                        } else {
                            println!("Tarefa marcada como concluida!");
                        }
                    }
                    _ => println!("Indice invalido!"),
                }
            }
            Ok(5) => {
                println!("Saindo...");
                break;
            }
            Ok(_) => println!("Opcao invalida!"),
            Err(_) => println!("Por favor, digite um numero valido!"),
        }

        let (concluidas, pendentes) = tasks.iter().fold((0, 0), |(c, p), task| {
            if task.completed {(c + 1, p)} else {(c, p + 1)}
        });

        println!("\nResumo:");
        println!("Total de tarefas: {}", tasks.len());
        println!("Tarefas concluidas: {}", concluidas);
        println!("Tarefas pendentes: {}", pendentes);
        println!("-----------------------------------\n");
    }
}