use std::fs;
use std::io;
use std::io::Write;
use std::sync::mpsc::TryRecvError;
use std::thread;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{ Constraint, Direction, Layout},
    widgets::Clear,
    Terminal,
};

mod interface;
mod networking;
mod parser;
mod state;

fn main() {
    let mut state = state::ApplicationState::new();

    state
        .history
        .push("help://".to_string());

    let mut content = networking::navigate(networking::UrlParsed::new(
        "help://",
    ));
    let mut p_height: usize = 0;
    let mut written_height: usize = 0;

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut scroll: usize = 0;
    let mut link_scroll: u16 = 0;
    let mut scroll_overflowed: bool = false;

    let stdin_channel = interface::input::spawn_stdin_channel();

    let mut update_ui = true;

    'main: loop {
        match stdin_channel.try_recv() {
            Ok(interface::input::Data::Command(interface::input::SignalType::Close)) => break 'main,
            Ok(interface::input::Data::Command(interface::input::SignalType::ScrollU)) => {
                if scroll != 0 {
                    scroll -= 1;
                }
                update_ui = true;
                scroll_overflowed = false;
            }
            Ok(interface::input::Data::Command(interface::input::SignalType::ScrollD)) => {
                if !scroll_overflowed {
                  scroll += 1;
                }
                update_ui = true;
            }
            Ok(interface::input::Data::Command(interface::input::SignalType::ScrollLU)) => {
                if link_scroll != 0 {
                    link_scroll -= 1;
                }
                update_ui = true;
            }
            Ok(interface::input::Data::Command(interface::input::SignalType::ScrollLD)) => {
                link_scroll += 1;
                update_ui = true;
            }
            Ok(interface::input::Data::Command(interface::input::SignalType::Go)) => {
                let redirect_link: String;
                if state.mode == state::Mode::Editing {
                    redirect_link = state.curr_input.clone();
                    state.curr_input.clear();
                    state.mode = state::Mode::Normal;
                } else {
                    redirect_link = parser::extract_link(content.as_str(), link_scroll, &state);
                }
                let url = networking::UrlParsed::new(redirect_link.as_str());
                state
                    .history
                    .push(url.get_request().trim_end_matches("/\r\n").to_string());
                let mut f = fs::File::create("content.txt").unwrap();
                f.write_all(format!("{:?}", url).as_bytes()).unwrap();
                content = networking::navigate(url);
                scroll = 0;
                link_scroll = 0;
                update_ui = true;
            }
            Ok(interface::input::Data::Command(interface::input::SignalType::NewURI)) => {
                state.mode = state::Mode::Editing;
                update_ui = true;
            }
            Ok(interface::input::Data::Command(interface::input::SignalType::HistoryBack)) => {
                if state.history.len() > 1 {
                    state.future.push(state.history.pop().unwrap());
                    let redirect_link = state.history[state.history.len() - 1].clone();
                    let url = networking::UrlParsed::new(redirect_link.as_str());
                    content = networking::navigate(url);
                    scroll = 0;
                    link_scroll = 0;
                    update_ui = true;
                }
            }
            Ok(interface::input::Data::Command(interface::input::SignalType::HistoryForward)) => {
                if state.future.len() > 0 {
                    let redirect_link = state.future.pop().unwrap();
                    state.history.push(redirect_link.clone());
                    let url = networking::UrlParsed::new(redirect_link.as_str());
                    content = networking::navigate(url);
                    scroll = 0;
                    link_scroll = 0;
                    update_ui = true;
                }
            }
            Ok(interface::input::Data::Char(c)) => {
                state.curr_input.push(c);
                update_ui = true;
            }
            Ok(interface::input::Data::Command(interface::input::SignalType::BackSpace)) => {
                state.curr_input.pop();
                update_ui = true;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Stdin thread disconnected!"),
        }
        if update_ui {
            update_ui = false;
            let styled_content = parser::parse_gemini(link_scroll,content.as_str().clone());
            terminal
                .draw(|f| {
                    let widget_main =
                        interface::ui::build_main(styled_content.clone(), scroll as u16);

                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(100)])
                        .split(f.size());

                    f.render_widget(widget_main.clone(), chunks[0]);
                    
                    //Contains correct height of rendered text
                    p_height = chunks[0].height as usize - 2;

                    //Check if paragraph overflowed
                    use std::io::BufRead;
                    let file_reader = std::fs::File::open("/tmp/tuitmp.txt").unwrap();
                    let file_reader = std::io::BufReader::new(file_reader);
                    written_height = file_reader.lines().last().unwrap().unwrap().trim().parse::<usize>().unwrap();
                      
                    if written_height - scroll < p_height {
                      scroll_overflowed = true;
                      //println!("p:{} w:{}", p_height, written_height);
                      //println!("DE");
                    }
                    if state.mode == state::Mode::Editing {
                        let popup = interface::popup::centered_rect(90, 90, f.size());

                        let widget_input = interface::ui::build_input(state.curr_input.clone());
                        f.render_widget(Clear, popup);
                        f.render_widget(widget_input, popup);
                    }
                })
                .unwrap();
        }
        thread::sleep(std::time::Duration::from_millis(20));
    }
}
