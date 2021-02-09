pub struct Status {
    f_digit_code: u8,
    s_digit_code: u8,
    meta: String,
}

impl Status {
    pub fn new(status: String) -> Self {
        let tokens: Vec<&str> = status.splitn(2, " ").collect();
        let status_code = tokens[0];
        if status_code.len() != 2 {
            panic!("Invalid status code recieved from server!");
        }
        Status {
            f_digit_code: status_code.chars().nth(0).unwrap().to_digit(10).unwrap() as u8,
            s_digit_code: status_code.chars().nth(1).unwrap().to_digit(10).unwrap() as u8,
            meta: String::from(tokens[1]),
        }
    }

    pub fn is_ok(&self) -> bool {
        if self.f_digit_code == 2 {
            true
        } else {
            false
        }
    }
}
