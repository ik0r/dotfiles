use std::path::Path;

use regex::Regex;

use crate::tasks::Context;
use crate::ui;
use crate::utils;

pub fn append_dotvim_group(group: &str, ctx: &Context) -> anyhow::Result<()> {
  let conf = ctx.home_dir.join(".vimrc.plugins.before");
  let content = utils::read_to_string_if_exists(&conf)?;

  let group_needle = format!("'{}'", group);
  if content.contains(group_needle.as_str()) {
    return Ok(());
  }

  let assign_re =
    Regex::new(r"(?im)^([ \t]*)let[ \t]+g:dotvim_groups[ \t]*=[ \t]*\[(.*)\][ \t]*$")?;
  if let Some(caps) = assign_re.captures(content.as_str()) {
    let indent = caps.get(1).map(|m| m.as_str()).unwrap_or("");
    let inner = caps.get(2).map(|m| m.as_str()).unwrap_or("");

    let mut items = inner
      .split(',')
      .map(|s| s.trim())
      .filter(|s| !s.is_empty())
      .map(|s| s.to_string())
      .collect::<Vec<_>>();
    items.push(group_needle);

    let new_line = format!("{}let g:dotvim_groups = [{}]", indent, items.join(", "));
    let replaced = assign_re.replace(&content, new_line.as_str()).to_string();

    if ctx.dry_run {
      return Ok(());
    }

    if let Some(parent) = conf.parent() {
      utils::ensure_dir(parent)?;
    }
    utils::atomic_write_string(&conf, replaced.as_str())?;
    return Ok(());
  }

  if ctx.dry_run {
    return Ok(());
  }

  let mut new_content = content;
  if !new_content.ends_with('\n') && !new_content.is_empty() {
    new_content.push('\n');
  }
  new_content.push_str(format!("let g:dotvim_groups = ['{}']\n", group).as_str());
  utils::atomic_write_string(&conf, new_content.as_str())?;
  Ok(())
}

pub fn ensure_neovim_python_support(ctx: &Context) -> anyhow::Result<()> {
  if !utils::program_exists("nvim") {
    return Ok(());
  }

  if is_gentoo() && Path::new("/etc/gentoo-release").exists() {
    ui::tip("You are using Gentoo Linux.");
    ui::tip("You should enable *python* USE flag for neovim, and reinstall neovim.");
    ui::tip("Then dev-python/neovim-python-client will be installed automatically.");
    ui::tip("Also you can run '[sudo] emerge -a dev-python/neovim-python-client' manually.");
    return Ok(());
  }

  if !utils::program_exists("pip")
    && !utils::program_exists("pip2")
    && !utils::program_exists("pip3")
  {
    anyhow::bail!("You must have installed pip or pip2 or pip3 for installing python packages.");
  }

  if utils::program_exists("pip2") {
    ui::info("Installing python2 neovim package ...");
    let status = utils::run_status(
      "pip2",
      &["install", "--user", "--upgrade", "neovim"],
      None,
      &[],
      false,
      ctx.dry_run,
    )?;
    if !status.success() {
      anyhow::bail!("pip2 install neovim failed");
    }
  } else if utils::program_exists("pip") {
    ui::info("Installing python2 neovim package ...");
    let status = utils::run_status(
      "pip",
      &["install", "--user", "--upgrade", "neovim"],
      None,
      &[],
      false,
      ctx.dry_run,
    )?;
    if !status.success() {
      anyhow::bail!("pip install neovim failed");
    }
  }

  if utils::program_exists("pip3") {
    ui::info("Installing python3 neovim package ...");
    let status = utils::run_status(
      "pip3",
      &["install", "--user", "--upgrade", "neovim"],
      None,
      &[],
      false,
      ctx.dry_run,
    )?;
    if !status.success() {
      anyhow::bail!("pip3 install neovim failed");
    }
  }

  ui::success("Successfully installed neovim python client.");
  Ok(())
}

fn is_gentoo() -> bool {
  let output = std::process::Command::new("uname").arg("-a").output();
  let Ok(output) = output else {
    return false;
  };
  if !output.status.success() {
    return false;
  }
  String::from_utf8_lossy(&output.stdout)
    .to_lowercase()
    .contains("gentoo")
}
