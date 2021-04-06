use std::io;
use std::sync::mpsc::TryRecvError;
use std::thread;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Terminal,
};
use std::fs;
use std::io::Write;

mod interface;
mod networking;
mod parser;
mod state;

fn main() {
    let mut state =  state::ApplicationState::new();

    state.history.push("gemini.circumlunar.space".to_string());
    
    let mut content = networking::navigate(networking::UrlParsed::new("gemini.circumlunar.space")
);
    let mut f = fs::File::create("content.txt").unwrap();
    f.write_all(content.as_str().as_bytes()).unwrap();

    let line_count = content.as_bytes().iter().filter(|&&c| c == b'\n').count();
    let mut p_block_size: usize = 0;
    let mut decrement_lscroll: bool = false;

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut scroll: usize = 0;
    let mut link_scroll: u16 = 0;

    let stdin_channel = interface::input::spawn_stdin_channel();

    let mut update_ui = true;

    'main: loop {
        match stdin_channel.try_recv() {
            Ok(interface::input::SignalType::Close) => break 'main,
            Ok(interface::input::SignalType::ScrollU) => {
                if scroll != 0 {
                    scroll -= 1;
                }
                update_ui = true;
            }
            Ok(interface::input::SignalType::ScrollD) => {
                if scroll < line_count - p_block_size + 5 {
                    scroll += 1;
                }
                update_ui = true;
            }
            Ok(interface::input::SignalType::ScrollLU) => {
                if link_scroll != 0 {
                    link_scroll -= 1;
                }
                update_ui = true;
            }
            Ok(interface::input::SignalType::ScrollLD) => {
                link_scroll += 1;
                update_ui = true;
            }
            Ok(interface::input::SignalType::Go) => {
                let url = networking::UrlParsed::new(parser::extract_link(content.as_str(), link_scroll, &state).as_str());
                thread::sleep_ms(5000);
                content =
                    networking::navigate(url);
                scroll = 0;
                link_scroll = 0;
                update_ui = true;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Stdin thread disconnected!"),
        }
        if update_ui {
            update_ui = false;

            let styled_content = parser::parse(link_scroll, scroll as u16, content.as_str());
            let widget = interface::ui::build(styled_content.clone(), scroll as u16);
            terminal
                .draw(|f| {
                    if decrement_lscroll {
                        link_scroll -= 1;
                    }
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(1), Constraint::Percentage(99)])
                        .split(f.size());
                    p_block_size = chunks[0].height as usize;
                    f.render_widget(widget.clone(), f.size());
                })
                .unwrap();
        }
        thread::sleep(std::time::Duration::from_millis(20));
    }
}
