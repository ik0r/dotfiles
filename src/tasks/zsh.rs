use std::path::Path;

use crate::tasks::Context;
use crate::ui;
use crate::utils;

pub fn must_python_pip(_ctx: &Context) -> anyhow::Result<()> {
  if !utils::program_exists("pip")
    && !utils::program_exists("pip2")
    && !utils::program_exists("pip3")
  {
    anyhow::bail!("You must have installed pip or pip2 or pip3 for installing python packages.");
  }
  Ok(())
}

pub fn ensure_default_shell_is_zsh(ctx: &Context) -> anyhow::Result<()> {
  let current_shell = std::env::var("SHELL").unwrap_or_default();
  let current_shell_base = Path::new(current_shell.as_str())
    .file_name()
    .unwrap_or_default()
    .to_string_lossy()
    .to_string();
  if current_shell_base == "zsh" {
    return Ok(());
  }

  if !utils::program_exists("chsh") {
    ui::error("I can't change your shell automatically because this system does not have chsh.");
    ui::error("Please manually change your default shell to zsh!");
    return Ok(());
  }

  let shells = std::fs::read_to_string("/etc/shells").unwrap_or_default();
  let zsh_path = shells
    .lines()
    .filter(|line| line.trim_end().ends_with("/zsh"))
    .last()
    .map(|s| s.trim().to_string());

  let Some(zsh_path) = zsh_path else {
    ui::error("Can't find zsh path in /etc/shells.");
    return Ok(());
  };

  ui::info("Time to change your default shell to zsh!");
  let status = ctx.run_status("chsh", &["-s", zsh_path.as_str()], None, &[], false)?;
  if !status.success() {
    ui::error("Failed to change default shell to zsh. Please do it manually.");
  }
  Ok(())
}
