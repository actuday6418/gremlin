use rustls::Session;
use std::io::{Read, Write};
use termion::{
    event::Key, input::MouseTerminal, input::TermRead, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Terminal,
};

mod networking;

fn navigate(url: networking::UrlParser) {
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

    while client.wants_read() {
        client.read_tls(&mut socket).unwrap();
        client.process_new_packets().unwrap();
    }
    let mut data = Vec::new();
    let _ = client.read_to_end(&mut data);

    let status = networking::status::Status::new(String::from(String::from_utf8_lossy(&data)));

    if status.is_ok() {
        client.read_tls(&mut socket).unwrap();
        client.process_new_packets().unwrap();
        let mut data = Vec::new();
        let _ = client.read_to_end(&mut data);

        println!("{}", String::from_utf8_lossy(&data));
    } else {
        panic!("Server returned error status!");
    }
}

fn main() {
    navigate(networking::UrlParser::new("gemini.circumlunar.space")); /*
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
