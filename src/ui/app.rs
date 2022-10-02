use std::io;
use rand::{thread_rng, Rng};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::text::{Spans, Text};
use tui::widgets::ListItem;


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}
fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();



    let list = List::new((vec!["TODO make app","Test app","Publish it"]).iter().map(|i|{
        ListItem::new(Spans::from(vec![Span::styled(
            if thread_rng().gen_bool(1.0 / 2.0) { " ".to_string() } else { " ".to_string() }+i.clone(),
            Style::default(),
        )]))
    }).collect::<Vec<_>>()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Forgotten todos")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
    );
    f.render_widget(list,size)
}