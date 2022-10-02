use crate::subcommands::list;
use clap::Command;
use clap::{command, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
#[derive(Parser)]
#[command(author = "OlshaMB", version = "0.5", about = "A todo list app that indexes your app to find TODO:'s", long_about = None, propagate_version = true)]
pub struct Arguments {
    #[clap(subcommand)]
    command: Commands,
}
#[enum_dispatch(Commands)]
pub trait SubCommandWithFunction {
    fn on_use(&self, args: &Arguments);
}

#[derive(Subcommand)]
#[enum_dispatch]
enum Commands {
    //UI(ui::UI),
    List(list::List),
}
pub fn use_command(args: Arguments) {
    args.command.on_use(&args)
}
