use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use termion::event::Key;
use std::io;
use termion::input::TermRead;

pub enum SignalType {
    Close,
    ScrollU,
    ScrollD,
    ScrollLU,
    ScrollLD,
}

pub fn spawn_stdin_channel() -> Receiver<SignalType> {
    let (tx, rx) = mpsc::channel::<SignalType>();
    thread::spawn(move || loop {
        let stdin = io::stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('c') => tx.send(SignalType::Close).unwrap(),
                Key::Up => tx.send(SignalType::ScrollU).unwrap(),
                Key::Down => tx.send(SignalType::ScrollD).unwrap(),
                Key::Ctrl('l') => tx.send(SignalType::ScrollLU).unwrap(),
                Key::Ctrl('k') => tx.send(SignalType::ScrollLD).unwrap(),
                _ => {}
            }
        }
    });
    rx
}