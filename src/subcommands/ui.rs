use std::io;
use clap::Args;
use crate::{Arguments, SubCommandWithFunction};
use crate::ui::app::run_app;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[derive(Args)]
pub struct UI{

}
impl SubCommandWithFunction for UI{
    fn on_use(&self, args: &Arguments) {
        enable_raw_mode().expect("Err: Raw mode fault");
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        // create app and run it
        let res = run_app(&mut terminal);

        // restore terminal
        disable_raw_mode();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        terminal.show_cursor();

        if let Err(err) = res {
            println!("{:?}", err)
        }

    }
}