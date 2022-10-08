use crate::todos_search::todos_search;
use crate::{Arguments, SubCommandWithFunction};
use clap::{Args, ValueEnum};
use crossterm::style::Stylize;
use regex::{Regex, Replacer};
use std::borrow::Borrow;
use std::path::{Path, PathBuf};
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CheckMarkType {
    Nerd,
    Emoji,
    UTF,
    Dash,
    Circle,
}

#[derive(Args)]
#[command(about = "List all your todos")]
pub struct List {
    #[arg(short, long, help = "files/dirs to ignore")]
    ignore: Vec<PathBuf>,
    #[arg(help = "A path where to search")]
    path: Option<PathBuf>,
    #[arg(
        short = 'c',
        long = "use_cl",
        help = "Replaces content of todo item with whole line where todo is located",
        default_value_t = false
    )]
    use_content_line_as_content: bool,
    #[arg(
        short = 'l',
        long = "cl",
        help = "Shows whole line where todo is located after todo item",
        default_value_t = false
    )]
    display_content_line: bool,
    #[arg(
        short = 'n',
        long = "no-color",
        help = "A option to disable color",
        default_value_t = false
    )]
    no_color: bool,
    #[arg(
        short = 't',
        long = "type",
        help = "A option to chose type of the checkmark to display",
        value_enum,
        default_value_t= CheckMarkType::Nerd,
    )]
    checkmark_type: CheckMarkType,
}
impl SubCommandWithFunction for List {
    fn on_use(&self, args: &Arguments) {
        for todo in todos_search(
            if self.ignore.is_empty() {
                vec![]
            } else {
                self.ignore.clone()
            },
            self.path.clone().unwrap(),
        ) {
            let checkmark = match self.checkmark_type {
                CheckMarkType::Circle => "•",
                CheckMarkType::Nerd => "",
                CheckMarkType::Emoji => "✅",
                CheckMarkType::Dash => "-",
                CheckMarkType::UTF => "✔",
            };
            let mut format_string = format!(
                "{} {} - {}:{}:{}{}",
                checkmark,
                if !self.use_content_line_as_content {
                    &todo.content
                } else {
                    &todo.line_content
                },
                &todo.path.dark_cyan(),
                &todo.line.to_string().dark_blue(),
                &todo.col.to_string().dark_yellow(),
                (if self.display_content_line {
                    "\n".to_string() + todo.line_content.clone().trim_start()
                } else {
                    "".to_string()
                })
                .dark_grey()
            )
            .to_string();
            if self.no_color {
                format_string = Regex::new(r"\u001b\[.*?m")
                    .unwrap()
                    .replace_all(format_string.as_str(), "")
                    .to_string();
            }
            println!("{}", format_string);
        }
    }
}
