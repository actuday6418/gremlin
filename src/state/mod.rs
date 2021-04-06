pub struct ApplicationState {
    pub history: Vec<String>,
    pub future: Vec<String>,
}

impl ApplicationState {
    pub fn new() -> Self {
        ApplicationState {
            history: Vec::new(),
            future: Vec::new(),
        }
    }
}