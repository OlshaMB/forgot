use regex::{Match, Regex};
use std::path::{Path, PathBuf};
use std::{fs::{read_dir, read_to_string}, io};
#[derive(Default)]
pub struct Todo {
    pub path: String,
    pub line: usize,
    pub col: usize,
    pub content: String,
    pub line_content: String
}
pub fn find_todos_in_files(path: &Path,todos: &mut Vec<Todo>){
    let text: String = read_to_string(path).unwrap_or("".to_string());
    let lines = text.lines().enumerate();
    let search_regex = Regex::new(r"todo!?\(\x{22}?(.*)?\x{22}?\)|(?i)TODO(?-i):? *(.*)|(?i)NotImplemented(?:Erro?r?)?(?-i)\(?\x{22}?(.*)?\x{22}?\)").unwrap();
    for (i, line) in lines {
        for capture in search_regex.captures_iter(line) {
            let mut todo = Todo::default();
            todo.line = i + 1;
            todo.col = capture.get(0).unwrap().start();
            todo.path = path.to_str().unwrap().to_string();
            todo.line_content = line.to_string();
            //println!("[DEBUG]{}", if capture.get(2).is_some() {capture.get(2).unwrap().as_str().to_string()} else {capture.len().to_string()});
            let content = capture.iter().enumerate().filter(|el|{
                if el.0==0 { return false; };
                return el.1.is_some();
            }).map(|el| el.1).nth(0);
            if content.is_some() {

                todo.content = if content.unwrap().is_some() { content.unwrap().unwrap().as_str().to_string() } else { "todo".to_string() }
            } else {
                todo.content = "todo".to_string();
            }
            todos.push(todo);
        }
    }
}
pub fn index_dirs(path: &Path, ignore: &Vec<PathBuf>, todos: &mut Vec<Todo>){
    if ignore.contains(&path.to_path_buf()) {
        return;
    }
    if path.is_dir(){
        let posibly_dir_entries = path.read_dir();
        if posibly_dir_entries.is_ok() {
            for entry in posibly_dir_entries.unwrap() {
                if entry.is_ok() {
                    index_dirs(&entry.unwrap().path().to_path_buf(), &ignore, todos)
                }
            }
        }
    } else {
        find_todos_in_files(path, todos)
    }
}
pub(crate) fn todos_search(ignore: Vec<PathBuf>, path: PathBuf) -> Vec<Todo> {
    let mut todos: Vec<Todo> = vec![];
    index_dirs(
        path.as_path(),
        &ignore,
        &mut todos
    );
    return todos;
}
