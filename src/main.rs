

use clap::{Parser};

use crate::args::*;
mod args;
mod subcommands;
//mod ui;
mod todos_search;

fn main() {
    let args = Arguments::parse();
    use_command(args)
}
