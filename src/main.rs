use std::io;
use std::sync::mpsc::TryRecvError;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Layout, Direction},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Terminal,
};

mod interface;
mod networking;

fn main() {
    let content = networking::navigate(networking::UrlParser::new("gemini.circumlunar.space"));
    let mut line_count = content.as_bytes().iter().filter(|&&c| c == b'\n').count();
    let mut p_block_size: usize = 0;

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut scroll: usize = 0;

    let stdin_channel = interface::input::spawn_stdin_channel();

    'main: loop {
        match stdin_channel.try_recv() {
            Ok(interface::input::SignalType::Close) => break 'main,
            Ok(interface::input::SignalType::ScrollU) => {
                if scroll != 0 {
                    scroll -= 1;
                }
            }
            Ok(interface::input::SignalType::ScrollD) => {
                if scroll < line_count - p_block_size +5 {
                scroll += 1;
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Stdin thread disconnected!"),
        }
        terminal
            .draw(|f| {
                let content_w = interface::ui::ret(scroll as u16, content.clone());
                let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(1), Constraint::Percentage(90)])
                .split(f.size());
                p_block_size = chunks[0].height as usize;
                f.render_widget(content_w.0, chunks[0]);
                f.render_widget(content_w.1, chunks[1]);

            })
            .unwrap();
    }
}
