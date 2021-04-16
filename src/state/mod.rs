#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Editing,
}

pub struct ApplicationState {
    pub history: Vec<String>,
    pub future: Vec<String>,
    pub curr_input: String,
    pub mode: Mode,
}

impl ApplicationState {
    pub fn new() -> Self {
        ApplicationState {
            history: Vec::new(),
            future: Vec::new(),
            mode: Mode::Normal,
            curr_input: String::from(""),
        }
    }
}
