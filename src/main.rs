use std::io;
use std::io::{Write, Read};
use termion::{
    event::Key, input::MouseTerminal, input::TermRead, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    //layout::{Constraint, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Terminal,
};
use rustls::{RootCertStore, Certificate, ServerCertVerified, TLSError, ServerCertVerifier, Session};
use webpki::DNSNameRef;

struct DummyVerifier { }

impl DummyVerifier {
    fn new() -> Self {
        DummyVerifier { }
    }
}

impl ServerCertVerifier for DummyVerifier {
    fn verify_server_cert(
        &self,
        _: &RootCertStore,
        _: &[Certificate],
        _: DNSNameRef,
        _: &[u8]
    ) -> Result<ServerCertVerified, TLSError> {
        return Ok(ServerCertVerified::assertion()); 
    }
}

fn navigate(url: &str) {
    let mut config = rustls::ClientConfig::new();
    let mut config2 = rustls::DangerousClientConfig {cfg: &mut config};
    let dummy_verifier = std::sync::Arc::new(DummyVerifier::new());
    config2
        .set_certificate_verifier(dummy_verifier);
    let shared_cfg = std::sync::Arc::new(config);
    let dns_name = webpki::DNSNameRef::try_from_ascii_str("gemini.circumlunar.space").unwrap();
    let mut client = rustls::ClientSession::new(&shared_cfg, dns_name);
    let mut socket = std::net::TcpStream::connect("gemini.circumlunar.space:1965").unwrap();
    let mut stream = rustls::Stream::new(&mut client, &mut socket);
    
    let request = "gemini://".to_string() + url + "/\r\n";

    stream.write_all(request.as_bytes()).unwrap();

    while client.wants_read() {
        client.read_tls(&mut socket).unwrap();
        client.process_new_packets().unwrap();
    }
    let mut data = Vec::new();
    let _ = client.read_to_end(&mut data);

    let status =  String::from_utf8_lossy(&data);

    //println!("{}", status);

    client.read_tls(&mut socket).unwrap();
    client.process_new_packets().unwrap();
    let mut data = Vec::new();
    let _ = client.read_to_end(&mut data);

    println!("{}", String::from_utf8_lossy(&data));
}

fn main() {

    navigate("gemini.circumlunar.space");/*
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    'main: loop {
        terminal
            .draw(|f| {
                let block = Block::default()
                    .border_style(Style::default().fg(Color::Green))
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        "Gremlin",
                        Style::default().fg(Color::LightCyan),
                    ))
                    .border_type(BorderType::Rounded);
                f.render_widget(block.clone(), f.size());
            })
            .unwrap();
        for c in io::stdin().keys() {
            match c.unwrap() {
                Key::Ctrl('c') => break 'main,
                _ => {}
            }
        }
    }*/
}
