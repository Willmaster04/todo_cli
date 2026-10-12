use std::fs::File;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::Path;

use crate::todo::{Todo, TodoStatus};

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

    let numeric_status = match todo.status {
        TodoStatus::Done => 1,
        TodoStatus::NotDone => 0
    };

    write!(file, "{} | {}\n", todo.name, numeric_status)
        .expect("failed to writeln");
}

pub(crate) fn get_all_todos_from_file() -> Vec<Todo> {
    create_file_if_not_exists();

    read_to_string(FILE_PATH)
        .expect("failed to read todos file")
        .lines()
        .map(|line| {
            let (name, status) = line
                .rsplit_once(" | ")
                .expect("invalid todo item");

            let status = match status {
                "1" => TodoStatus::Done,
                "0" => TodoStatus::NotDone,
                _ => panic!("Invalid todo status: {status}"),
            };
            
            Todo::new_from_file(name, status)

        })
        .collect()
}

pub(crate) fn update_todo(todo_number: usize) -> io::Result<()> {
    create_file_if_not_exists();

    let contents = read_to_string(FILE_PATH)?;
    let mut lines: Vec<String> = contents.lines().map(str::to_owned).collect();
    let line = lines
        .get_mut(todo_number)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "todo number is out of range"))?;

    let (name, status) = line
        .rsplit_once(" | ")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid todo item"))?;

    let new_status = match status {
        "1" => "0",
        "0" => "1",
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid todo status")),
    };

    *line = format!("{name} | {new_status}");

    let mut updated_contents = lines.join("\n");
    if contents.ends_with('\n') {
        updated_contents.push('\n');
    }

    std::fs::write(FILE_PATH, updated_contents)
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