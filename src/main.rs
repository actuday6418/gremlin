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
use std::thread;

mod interface;
mod networking;
mod parser;

fn main() {
    let mut content = networking::navigate(networking::UrlParser::new("gemini.circumlunar.space"));

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
    let mut styled_content = parser::parse(link_scroll, scroll as u16, content.as_str());
    let mut update_styled = true;

    'main: loop {
        match stdin_channel.try_recv() {
            Ok(interface::input::SignalType::Close) => break 'main,
            Ok(interface::input::SignalType::ScrollU) => {
                if scroll != 0 {
                    scroll -= 1;
                }
                update_styled = true;
            }
            Ok(interface::input::SignalType::ScrollD) => {
                if scroll < line_count - p_block_size + 5 {
                    scroll += 1;
                }
                update_styled = true;
            }
            Ok(interface::input::SignalType::ScrollLU) => {
                if link_scroll != 0 {
                    link_scroll -= 1;
                }
                update_styled = true;
            }
            Ok(interface::input::SignalType::ScrollLD) => {
                link_scroll += 1;
                update_styled = true;
            }
            Ok(interface::input::SignalType::Go) => {
                scroll = 0;
                link_scroll = 0;
                update_styled = true;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Stdin thread disconnected!"),
        }
        if update_styled {
            styled_content = parser::parse(link_scroll, scroll as u16, content.as_str());
        }
        if update_styled {
            update_styled = false;
            terminal
                .draw(|f| {
                    let w = Paragraph::new(styled_content.clone())
                        .style(Style::default())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .style(Style::default())
                                .title(Span::styled("Gremlin", Style::default())),
                        )
                        .alignment(Alignment::Left)
                        .wrap(Wrap { trim: true })
                        .scroll((scroll as u16, 0));
                    if decrement_lscroll {
                        link_scroll -= 1;
                    }
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(1), Constraint::Percentage(99)])
                        .split(f.size());
                    p_block_size = chunks[0].height as usize;
                    f.render_widget(w, f.size());
                })
                .unwrap();
        }
        thread::sleep(std::time::Duration::from_millis(20));
    }
}
