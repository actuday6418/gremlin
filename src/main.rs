use rustls::Session;
use std::io;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;
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

mod networking;

enum SignalType {
    close,
    scroll_u,
    scroll_d,
}

fn navigate(url: networking::UrlParser) -> String {
    let mut config = rustls::ClientConfig::new();
    let mut config2 = rustls::DangerousClientConfig { cfg: &mut config };
    let certificate_verifier = std::sync::Arc::new(networking::CertVerifier::new());
    config2.set_certificate_verifier(certificate_verifier);
    let shared_cfg = std::sync::Arc::new(config);
    let dns_name = webpki::DNSNameRef::try_from_ascii_str(url.get_name()).unwrap();
    let mut client = rustls::ClientSession::new(&shared_cfg, dns_name);
    let mut socket =
        std::net::TcpStream::connect(url.get_name().to_string() + url.get_port()).unwrap();
    let mut stream = rustls::Stream::new(&mut client, &mut socket);
    stream.write_all(url.get_request().as_bytes()).unwrap();

    let mut data = Vec::new();
    let _ = stream.read_to_end(&mut data);
    let data = String::from(String::from_utf8_lossy(&data));
    let mut status_string = String::new();
    let mut content_string: String;
    let mut chars = data.chars();
    let mut no_chars: i32 = 0;
    loop {
        no_chars += 1;
        let c = chars.next().unwrap();
        if c == '\n' {
            break;
        } else {
            status_string.push(c);
        }
    }
    content_string = data;
    content_string.drain(..no_chars as usize);
    let status = networking::status::Status::new(status_string);
    if status.is_ok() {
        content_string
    } else {
        panic!("Server returned error status!");
    }
}

fn main() {
    let content = navigate(networking::UrlParser::new("gemini.circumlunar.space"));
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut scroll: u16 = 0;

    let stdin_channel = spawn_stdin_channel();

    'main: loop {
        match stdin_channel.try_recv() {
            Ok(SignalType::close) => break 'main,
            Ok(SignalType::scroll_u) => {
                if scroll != 0 {
                    scroll -= 1;
                }
            }
            Ok(SignalType::scroll_d) => {
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

fn spawn_stdin_channel() -> Receiver<SignalType> {
    let (tx, rx) = mpsc::channel::<SignalType>();
    thread::spawn(move || loop {
        let stdin = io::stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('c') => tx.send(SignalType::close).unwrap(),
                Key::Up => tx.send(SignalType::scroll_u).unwrap(),
                Key::Down => tx.send(SignalType::scroll_d).unwrap(),
                _ => {}
            }
        }
    });
    rx
}
