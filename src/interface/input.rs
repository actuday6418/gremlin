use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use termion::event::Key;
use std::io;
use termion::input::TermRead;
use crate::state;

pub enum SignalType {
    Close,
    ScrollU,
    ScrollD,
    ScrollLU,
    ScrollLD,
    Go,
    HistoryBack,
    HistoryForward,
    NewURI,
    BackSpace,
}

pub enum Data {
    Command(SignalType),
    Char(char),
}

pub fn spawn_stdin_channel() -> Receiver<Data> {
    let (tx, rx) = mpsc::channel::<Data>();
    let mut input_mode: state::Mode = state::Mode::Normal;

    thread::spawn(move || loop {
        let stdin = io::stdin();
        for c in stdin.keys() {
            if input_mode == state::Mode::Normal {
            match c.unwrap() {
                Key::Ctrl('c') => tx.send(Data::Command(SignalType::Close)).unwrap(),
                Key::Up => tx.send(Data::Command(SignalType::ScrollU)).unwrap(),
                Key::Down => tx.send(Data::Command(SignalType::ScrollD)).unwrap(),
                Key::Ctrl('k') => tx.send(Data::Command(SignalType::ScrollLU)).unwrap(),
                Key::Ctrl('l') => tx.send(Data::Command(SignalType::ScrollLD)).unwrap(),
                Key::Char('\n') => tx.send(Data::Command(SignalType::Go)).unwrap(),
                Key::Alt('k') => tx.send(Data::Command(SignalType::HistoryBack)).unwrap(),
                Key::Alt('l') => tx.send(Data::Command(SignalType::HistoryForward)).unwrap(),
                Key::Alt('n') => {
                    input_mode = state::Mode::Editing;
                    tx.send(Data::Command(SignalType::NewURI)).unwrap();
                }
                _ => {}
            }
            } else {
                match c.unwrap() {
                    Key::Char('\n') => {
                        input_mode = state::Mode::Normal;
                        tx.send(Data::Command(SignalType::Go)).unwrap();
                    }
                    Key::Backspace => tx.send(Data::Command(SignalType::BackSpace)).unwrap(),
                    Key::Char(x) => tx.send(Data::Char(x)).unwrap(),
                    Key::Ctrl('c') => tx.send(Data::Command(SignalType::Close)).unwrap(),
                    _ => {}
                }
            }
        }
    });
    thread::sleep(std::time::Duration::from_millis(20));
    rx
}