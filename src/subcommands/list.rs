use crate::todos_search::todos_search;
use crate::{Arguments, SubCommandWithFunction};
use clap::Args;
use crossterm::style::Stylize;
use std::borrow::Borrow;
use std::path::{Path, PathBuf};

#[derive(Args)]
#[command(about = "List all your todos")]
pub struct List {
    #[arg(short, long, help = "files/dirs to ignore separated by comma(,)")]
    ignore: Option<String>,
}
impl SubCommandWithFunction for List {
    fn on_use(&self, args: &Arguments) {
        for todo in todos_search(
            self.ignore
                .clone()
                .unwrap_or("".to_string())
                .split(",")
                .map(|s| Path::new(s).to_path_buf())
                .collect::<Vec<PathBuf>>()
                .as_ref(),
        ) {
            println!(
                " {} - {}:{}:{}",
                todo.content.grey(),
                todo.path.red(),
                todo.line.to_string().blue(),
                todo.col.to_string().yellow()
            )
        }
    }
}
