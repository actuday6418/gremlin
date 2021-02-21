use std::io;
use termion::{
    event::Key, input::MouseTerminal, input::TermRead, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Terminal,
};
use std::sync::mpsc::TryRecvError;

mod networking;
mod interface;

fn main() {
    let content = networking::navigate(networking::UrlParser::new("gemini.circumlunar.space"));
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut scroll: u16 = 0;

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
                scroll += 1;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Stdin thread disconnected!"),
        }
        terminal
            .draw(|f| {
                let para = Paragraph::new(content.clone())
                    .style(Style::default())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default())
                            .title(Span::styled("Gremlin", Style::default())),
                    )
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
                    .scroll((scroll, 0));
                f.render_widget(para, f.size());
            })
            .unwrap();
    }
}


