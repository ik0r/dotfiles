use std::path::{Path, PathBuf};

use crate::tasks::Context;
use crate::tasks::git;
use crate::ui;
use crate::utils;

pub(super) fn install_fonts_source_code_pro(ctx: &Context) -> anyhow::Result<()> {
  let force = std::env::var("DOT_FORCE_FONTS_INSTALL")
    .ok()
    .filter(|v| !v.is_empty());
  if force.is_none() {
    let ignore = std::env::var("DOT_IGNORE_FONTS_INSTALL")
      .ok()
      .filter(|v| !v.is_empty());
    if ignore.is_some() {
      ui::info("Pass installing fonts according to DOT_IGNORE_FONTS");
      return Ok(());
    }
    let ssh = std::env::var("SSH_CONNECTION")
      .ok()
      .filter(|v| !v.is_empty());
    if ssh.is_some() {
      ui::info("Pass installing fonts according to SSH_CONNECTION");
      ui::tip("Maybe you should install the font *Source Code Pro* locally.");
      return Ok(());
    }
  }

  if !utils::is_mac() && !utils::is_linux() {
    anyhow::bail!("This support *Linux* and *Mac* only");
  }

  utils::must_program("git")?;
  ui::step("Installing font Source Code Pro ...");

  let cache_dir = ctx.app_path.join(".cache/source-code-pro");
  git::sync_repo(
    "https://github.com/adobe-fonts/source-code-pro.git",
    &cache_dir,
    Some("release"),
    ctx,
  )?;

  let ttf_dir = cache_dir.join("TTF");
  let fonts_dir = if utils::is_mac() {
    ctx.home_dir.join("Library/Fonts")
  } else {
    ctx.home_dir.join(".fonts")
  };
  if !ctx.dry_run {
    utils::ensure_dir(&fonts_dir)?;
  }

  for entry in walk_dir_files(&ttf_dir)? {
    let Some(name) = entry.file_name().map(|s| s.to_string_lossy().to_string()) else {
      continue;
    };
    let lower = name.to_lowercase();
    let ok = lower.ends_with(".ttf") || lower.ends_with(".otf") || lower.ends_with(".pcf.gz");
    if ok {
      utils::copy_file(&entry, &fonts_dir, ctx.dry_run)?;
    }
  }

  if utils::program_exists("fc-cache") {
    let _ = ctx.run_status_quiet(
      "fc-cache",
      &["-f", fonts_dir.to_string_lossy().as_ref()],
      None,
      &[],
      false,
    )?;
  }

  ui::success("Successfully installed Source Code Pro font.");
  Ok(())
}

fn walk_dir_files(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
  let mut out = Vec::new();
  if !root.exists() {
    return Ok(out);
  }
  let mut stack = vec![root.to_path_buf()];
  while let Some(dir) = stack.pop() {
    for entry in std::fs::read_dir(&dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        stack.push(path);
      } else if path.is_file() {
        out.push(path);
      }
    }
  }
  Ok(out)
}
