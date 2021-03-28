use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use directories::ProjectDirs;
use std::process;

pub struct TodoTxt {
    pub content: Vec<TodoItem>,
    pub file_path: PathBuf,
}

impl TodoTxt {
    pub fn new(name: &String) -> TodoTxt {
        load_todo_txt(name)
    }

    pub fn toggle(&mut self, index: usize) {
        self.content[index].not_done = !self.content[index].not_done;
    }

    pub fn sort(&mut self) {
        &self.content.sort_by(|a, b| b.not_done.cmp(&a.not_done));
    }

    pub fn save_to_disk(&self) {
        let mut todo_file = match File::create(&self.file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Problem Opening File for Writing: {}", e);
                process::exit(1);
            }
        };
        let mut all_items = String::new();
        for elem in &self.content {
            let mut to_disk_string: String;
            if elem.priority != ' ' {
                to_disk_string = format!("({}) {}\n", elem.priority, elem.text);
            } else {
                to_disk_string = format!("{}\n", elem.text);
            }

            if !elem.not_done {
                to_disk_string = format!("x {}", to_disk_string);
            }
            all_items.push_str(&to_disk_string[..]);
        }
        all_items = all_items.trim().to_string();
        match todo_file.write(all_items.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Problem Writing to File: {}", e);
                process::exit(1);
            }
        };
    }
}

pub struct TodoItem {
    pub text: String,
    pub priority: char,
    pub not_done: bool,
}

impl TodoItem {
    pub fn new(text: String) -> TodoItem {
        TodoItem {
            text,
            priority: ' ',
            not_done: true,
        }
    }
}

fn load_todo_txt(name: &String) -> TodoTxt {
    let proj_dirs = get_project_dir();
    let filename_with_extension = format!("{}.txt", name);
    create_dirs_that_does_not_exist(&proj_dirs);
    let filename_with_path = Path::new(proj_dirs.data_dir()).join(filename_with_extension);
    let open_file = open_todo_file(&filename_with_path);
    parse_file(open_file, filename_with_path)
}

fn get_project_dir() -> ProjectDirs {
    let project_dir = match ProjectDirs::from("dev", "jonasjohansson", "docket") {
        Some(dir) => dir,
        None => {
            eprintln!("Problem Parsing User Directories");
            process::exit(1);
        }
    };
    project_dir
}

fn create_dirs_that_does_not_exist(proj_dirs: &ProjectDirs) {
    if !proj_dirs.data_dir().is_dir() {
        match std::fs::create_dir_all(proj_dirs.data_dir()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Problem Reading or Creating Data Folders: {}", e);
                process::exit(1);
            }
        };
    };
}

fn open_todo_file(filename_with_path: &PathBuf) -> File {
    let open_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename_with_path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "Problem Opening or Creating {:?} : {}",
                filename_with_path, e
            );
            process::exit(1);
        }
    };
    open_file
}

fn parse_file(mut open_file: File, file_path: PathBuf) -> TodoTxt {
    let mut content = String::new();
    match open_file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Problem Reading From File: {}", e);
            process::exit(1);
        }
    };
    let mut todo = TodoTxt {
        content: Vec::new(),
        file_path,
    };
    for line in content.lines() {
        if line.len() > 0 {
            let mut text = String::from(line);
            let (first, rest) = text.split_at(1);
            let mut not_done = true;
            if first == "x" {
                not_done = false;
                text = String::from(rest.trim());
            }
            let item = TodoItem {
                text,
                not_done,
                priority: ' ',
            };
            todo.content.push(item)
        }
    }
    drop(open_file);
    todo.content.sort_by(|a, b| b.not_done.cmp(&a.not_done));
    todo
}

#[test]
fn open_and_close_file() {
    let todo = TodoTxt::new(&String::from("test"));
    todo.save_to_disk();
}

#[test]
fn open_and_close_does_not_add_item() {
    let mut todo = TodoTxt::new(&String::from("test"));
    todo.content.clear();
    let mut todo_item = TodoItem::new(String::from("test item"));
    todo_item.not_done = !todo_item.not_done;
    todo.content.push(todo_item);
    todo.content[0].not_done = !todo.content[0].not_done;
    todo.save_to_disk();
    let todo_again = load_todo_txt(&String::from("test"));
    assert_eq!(todo.content.len(), todo_again.content.len())
}
