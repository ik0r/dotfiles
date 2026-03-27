use owo_colors::OwoColorize;

pub fn step(msg: &str) {
  eprintln!("\n{}", format!("[→] {}", msg).yellow());
}

pub fn info(msg: &str) {
  eprintln!("{}", format!("[>] {}", msg).cyan());
}

pub fn success(msg: &str) {
  eprintln!("{}", format!("[✓] {}", msg).green());
}

pub fn error(msg: &str) {
  eprintln!("{}", format!("[✗] {}", msg).red().bold());
}

pub fn tip(msg: &str) {
  eprintln!("{}", format!("[!] {}", msg).red().bold());
}
