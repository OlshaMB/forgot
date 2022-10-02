use std::{fs, io};
use std::fmt::format;
use std::fs::DirEntry;
use std::ops::Index;
use std::path::{Path, PathBuf};
use crossterm::style::Stylize;
use regex::Regex;
#[derive(Default)]
pub struct Todo {
    pub path: String,
    pub line: usize,
    pub col: usize,
    pub content: String
}
//TODO: remove todos argument
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry,&mut Vec<Todo>), todos: &mut Vec<Todo>, ignore:&Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {

            let entry = entry?;
            let path = entry.path();
            //println!("{} - {}", path.to_str().unwrap(), ignore.contains(&path).to_string().bold().blue());
            if ignore.contains(&path) { continue }
            if path.is_dir() {
                visit_dirs(&path, cb, todos, &ignore)?;
            } else {
                cb(&entry, todos);
            }
        }
    }
    Ok(())
}
pub(crate) fn todos_search(ignore:&Vec<PathBuf>) -> Vec<Todo>{

    let mut todos:Vec<Todo> = vec![];
    let search_regex = Regex::new(r"TODO:? *(.*)|todo!?\(?\x{22}?(.*)?\x{22}?\)").unwrap();
    visit_dirs(Path::new("."), &|entr, todos| {

        let text:String = fs::read_to_string(entr.path()).unwrap_or("".to_string());
        for (i, line) in text.lines().enumerate() {
            for capture in search_regex.captures_iter(line) {
                let mut todo = Todo::default();
                todo.line = i + 1;
                todo.col = capture.get(0).unwrap().start();
                todo.path = entr.path().to_str().unwrap().to_string();
                if capture.get(1).is_some() {
                    todo.content = capture.get(1).unwrap().as_str().to_string()
                } else {
                    todo.content = "todo".to_string();
                }
                todos.push(todo);
            };
        }
    }, &mut todos, &ignore).expect("");
    return todos;
}