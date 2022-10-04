use crate::todos_search::todos_search;
use crate::{Arguments, SubCommandWithFunction};
use clap::Args;
use crossterm::style::Stylize;
use std::borrow::Borrow;
use std::path::{Path, PathBuf};

#[derive(Args)]
#[command(about = "List all your todos")]
pub struct List {
    #[arg(short, long, help = "files/dirs to ignore")]
    ignore: Vec<PathBuf>,
    #[arg(help="A path where to search")]
    path: Option<PathBuf>,
    #[arg(short='c', long="use_cl", help="Replaces content of todo item with whole line where todo is located", default_value_t=false)]
    use_content_line_as_content:bool,
    #[arg(short='l', long="cl", help="Shows whole line where todo is located after todo item", default_value_t=false)]
    display_content_line: bool
}
impl SubCommandWithFunction for List {
    fn on_use(&self, args: &Arguments) {
        for todo in todos_search(
            if self.ignore.is_empty() { vec![] } else { self.ignore.clone() },
            self.path.clone().unwrap_or(PathBuf::from("."))
        ) {

            println!(
                " {} - {}:{}:{}{}",
                if !self.use_content_line_as_content {&todo.content} else {&todo.line_content},
                &todo.path.dark_cyan(),
                &todo.line.to_string().dark_blue(),
                &todo.col.to_string().dark_yellow(),
                (if self.display_content_line
                {"\n".to_string() + todo.line_content.clone().trim_start()}
                else
                {"".to_string()})
                                    .dark_grey()
            )
        }
    }
}
