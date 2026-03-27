use inquire::{Confirm, InquireError, Select, Text};

pub fn confirm(msg: &str, default: bool) -> Result<bool, InquireError> {
  Confirm::new(msg).with_default(default).prompt()
}

pub fn select(msg: &str, options: Vec<&str>, default_index: usize) -> Result<String, InquireError> {
  Select::new(msg, options)
    .with_starting_cursor(default_index)
    .prompt()
    .map(String::from)
}

pub fn input(msg: &str, default: &str) -> Result<String, InquireError> {
  Text::new(msg).with_default(default).prompt()
}
