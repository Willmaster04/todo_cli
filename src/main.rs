use std::io;
mod todo_io;

fn main() {
    println!("=======================");
    println!("Welcome to the todo cli");
    println!("=======================");
    println!();

    println!("Enter a command: ");
    println!("1. Add a todo");
    println!("2. List todos");
    println!("3. Complete a todo");
    println!("4. Delete all todos");
    println!("5. Exit");

    let mut command = String::new();
    
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    let command: u8 = match command.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("Invalid command"),
    };

    match command {
        1 => add_todo(),
        2 => list_todos(),
        3 => complete_todo(),
        4 => delete_todos(),
        5 => exit(),
        _ => invalid_command()
    }
}

fn add_todo() {
    println!("Enter the todo: ");

    let mut todo_name = String::new();
    io::stdin()
        .read_line(&mut todo_name)
        .expect("Failed to read line");

    let new_todo = Todo::new(&todo_name); 

    println!("Todo added: {:?}", new_todo);

    todo_io::write_todo_to_file(&new_todo.name)
}

fn list_todos() {
    let todos = todo_io::get_all_todos_from_file();

    if todos.len() == 0 {
        println!("No todos found");
        return;
    }

    for todo in todos {
        println!("- {}", todo);
    }
}

fn delete_todos() {
    println!("Deleting all todos...");

    todo_io::delete_all_todos_from_file();
}

fn complete_todo() {   
    let todos = todo_io::get_all_todos_from_file();

    if todos.len() == 0 {
        println!("No todos found");
        return;
    }

    println!("What todo would you like to complete?");

    for (i, todo) in todos.iter().enumerate() {
        println!("{}. todo: {}", i, todo)
    }
    
    let mut chosen_todo = String::new();
    io::stdin()
        .read_line(&mut chosen_todo)
        .expect("Failed to read line");

    let todo_number: usize = match chosen_todo.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("Invalid command"),
    };

    println!("You chose to complete todo: {}", todos[todo_number]);

    todo_io::delete_todo_from_file(todo_number);
}

fn exit() {
    println!("Exiting...");
}

fn invalid_command() {
    println!("Invalid command");
}

#[derive(Debug)]
enum TodoStatus {
    Done,
    NotDone
}

#[derive(Debug)]
struct Todo {
    name: String,
    status: TodoStatus
}

impl Todo {
    fn new(name: &str) -> Todo {
        Todo {
            name: name.to_owned(),
            status: TodoStatus::NotDone
        }
    }
}