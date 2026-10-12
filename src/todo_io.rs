use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

use crate::Todo;
use crate::TodoStatus;

const FILE_PATH: &str = "./todos.txt";

fn create_file_if_not_exists() {
    let path_exists = Path::new(FILE_PATH).exists();

    if !path_exists {
        let _file = File::create(FILE_PATH).expect("Failed to create file");
    }
}

pub(crate) fn write_todo_to_file(todo: &Todo) {
    create_file_if_not_exists();

    let mut file = File::options()
        .append(true)
        .open(FILE_PATH)
        .expect("failed to open file");

    write!(file, "{} - {:?}\n", todo.name, todo.status)
        .expect("failed to writeln");
}

pub(crate) fn get_all_todos_from_file() -> Vec<Todo> {
    create_file_if_not_exists();

    read_to_string(FILE_PATH)
        .expect("failed to read todos file")
        .lines()
        .map(|line| {
            let (name, status) = line
                .rsplit_once(" - ")
                .expect("invalid todo item");

            let status = match status {
                "Done" => TodoStatus::Done,
                "NotDone" => TodoStatus::NotDone,
                _ => panic!("Invalid todo status: {status}"),
            };
            
            Todo::new_from_file(name, status)

        })
        .collect()
}

pub(crate) fn delete_todo_from_file(line_number: usize) {
    create_file_if_not_exists();

    let mut lines: Vec<String> = read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    lines.remove(line_number);

    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("failed to open file");

    write!(file, "{}", lines.join("\n")).expect("failed to write to file");
}

pub(crate) fn delete_all_todos_from_file() {
    create_file_if_not_exists();

    let file = File::options()
        .write(true)
        .open(FILE_PATH)
        .expect("failed to open file");

    file.set_len(0).expect("failed to truncate file");
}