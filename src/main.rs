use std::io;
use std::sync::mpsc::TryRecvError;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Terminal,
};

mod interface;
mod networking;

fn main() {
    let content = networking::navigate(networking::UrlParser::new("gemini.circumlunar.space"));

    let line_count = content.as_bytes().iter().filter(|&&c| c == b'\n').count();
    let mut p_block_size: usize = 0;

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut scroll: usize = 0;
    let mut link_scroll: u16 = 0;

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
                if scroll < line_count - p_block_size + 5 {
                    scroll += 1;
                }
            }
            Ok(interface::input::SignalType::ScrollLU) => {
                if link_scroll != 0 {
                    link_scroll -= 1;
                }
            }
            Ok(interface::input::SignalType::ScrollLD) => {
                link_scroll += 1;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Stdin thread disconnected!"),
        }
        terminal
            .draw(|f| {
                let (decrement_lscroll, mut ret) = interface::ui::ret(link_scroll, scroll as u16, content.as_str());
                if decrement_lscroll {
                    link_scroll -= 1;
                }
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(1), Constraint::Percentage(99)])
                    .split(f.size());
                p_block_size = chunks[0].height as usize;
                f.render_widget(ret.pop().unwrap(), f.size());
            })
            .unwrap();
    }
}
