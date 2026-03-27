use std::path::Path;

use crate::prompt;
use crate::tasks::Context;
use crate::tasks::git;
use crate::ui;
use crate::utils;

pub(super) fn install_git_alias(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  ui::step("Install git alias ...");

  let cache_dir = ctx.app_path.join("git/.cache/gitalias");
  git::sync_repo(
    "https://github.com/GitAlias/gitalias.git",
    &cache_dir,
    None,
    ctx,
  )?;

  let options = vec!["system", "global", "local", "worktree"];
  let selected = prompt::select("where do you select to install?", options, 0)?;

  let include_path = cache_dir.join("gitalias.txt");
  let include_path_str = include_path.to_string_lossy().to_string();

  match selected.as_str() {
    "system" => {
      let sudo = !utils::is_root();
      let status = ctx.run_status(
        "git",
        &[
          "config",
          "--system",
          "include.path",
          include_path_str.as_str(),
        ],
        None,
        &[],
        sudo,
      )?;
      if !status.success() {
        anyhow::bail!("git config --system failed");
      }
    }
    "global" => {
      let status = ctx.run_status(
        "git",
        &[
          "config",
          "--global",
          "include.path",
          include_path_str.as_str(),
        ],
        None,
        &[],
        false,
      )?;
      if !status.success() {
        anyhow::bail!("git config --global failed");
      }
    }
    "local" => {
      ui::info("run below commands in you git repo");
      eprintln!();
      eprintln!(
        "git config --local include.path {}",
        include_path.to_string_lossy()
      );
      eprintln!();
    }
    "worktree" => {
      ui::info("run below commands in you git repo");
      eprintln!();
      eprintln!(
        "git config --worktree include.path {}",
        include_path.to_string_lossy()
      );
      eprintln!();
    }
    _ => {}
  }

  ui::success("Successfully installed git alias.");
  Ok(())
}

pub(super) fn install_git_config(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  ui::step("Installing gitconfig ...");

  let src = ctx.app_path.join("git/gitconfig");
  let dst = ctx.home_dir.join(".gitconfig");
  ui::info(format!("Linking {} to {}", src.display(), dst.display()).as_str());
  ctx.symlink(&src, &dst)?;

  ui::info("Now config your name and email for git.");

  let user_now = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
  let user_name = prompt::input(
    format!("What's your git username? ({})", user_now).as_str(),
    user_now.as_str(),
  )?;
  let default_email = format!("{}@example.com", user_name);
  let user_email = prompt::input(
    format!("What's your git email? ({})", default_email).as_str(),
    default_email.as_str(),
  )?;

  let status1 = ctx.run_status(
    "git",
    &["config", "--global", "user.name", user_name.as_str()],
    None,
    &[],
    false,
  )?;
  if !status1.success() {
    anyhow::bail!("git config user.name failed");
  }
  let status2 = ctx.run_status(
    "git",
    &["config", "--global", "user.email", user_email.as_str()],
    None,
    &[],
    false,
  )?;
  if !status2.success() {
    anyhow::bail!("git config user.email failed");
  }

  ui::success("Successfully installed gitconfig.");
  Ok(())
}

pub(super) fn install_git_diff_so_fancy(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  ui::step("Installing git diff-so-fancy ...");

  let cache_dir = ctx.app_path.join(".cache/diff-so-fancy");
  git::sync_repo(
    "https://github.com/so-fancy/diff-so-fancy.git",
    &cache_dir,
    None,
    ctx,
  )?;

  ctx.symlink(
    &ctx.app_path.join("git/bin/git-dsf"),
    &cache_dir.join("git-dsf"),
  )?;
  ctx.symlink(
    &ctx.app_path.join("git/bin/git-dsfc"),
    &cache_dir.join("git-dsfc"),
  )?;
  ctx.symlink(
    &ctx.app_path.join("git/bin/git-lsp"),
    &cache_dir.join("git-lsp"),
  )?;

  ui::success("Successfully installed git diff-so-fancy.");
  ui::info(format!("Please add '{}' to your PATH.", cache_dir.display()).as_str());
  Ok(())
}

pub(super) fn install_git_difftool_kaleidoscope(ctx: &Context) -> anyhow::Result<()> {
  if !utils::is_mac() {
    anyhow::bail!("Only MAC is supported");
  }

  utils::must_program("git")?;
  utils::must_program("ksdiff")?;
  utils::must_file(Path::new(
    "/Applications/Kaleidoscope.app/Contents/MacOS/Kaleidoscope",
  ))?;

  ui::step("Config git's difftool to Kaleidoscope ...");
  ui::info("Config git's difftool to Kaleidoscope");

  let status1 = ctx.run_status(
    "git",
    &["config", "--global", "diff.tool", "Kaleidoscope"],
    None,
    &[],
    false,
  )?;
  if !status1.success() {
    anyhow::bail!("git config diff.tool failed");
  }
  let status2 = ctx.run_status(
    "git",
    &[
      "config",
      "--global",
      "difftool.Kaleidoscope.cmd",
      r#"ksdiff --partial-changeset --relative-path "$MERGED" -- "$LOCAL" "$REMOTE""#,
    ],
    None,
    &[],
    false,
  )?;
  if !status2.success() {
    anyhow::bail!("git config difftool cmd failed");
  }
  let status3 = ctx.run_status(
    "git",
    &["config", "--global", "difftool.prompt", "false"],
    None,
    &[],
    false,
  )?;
  if !status3.success() {
    anyhow::bail!("git config difftool.prompt failed");
  }

  ui::success("Successfully config git's difftool");
  Ok(())
}

pub(super) fn install_git_mergetool_kaleidoscope(ctx: &Context) -> anyhow::Result<()> {
  if !utils::is_mac() {
    anyhow::bail!("Only MAC is supported");
  }

  utils::must_program("git")?;
  utils::must_program("ksdiff")?;
  utils::must_file(Path::new(
    "/Applications/Kaleidoscope.app/Contents/MacOS/Kaleidoscope",
  ))?;

  ui::step("Config git's mergetool to Kaleidoscope ...");
  ui::info("Config git's mergetool to Kaleidoscope");

  let status1 = ctx.run_status(
    "git",
    &["config", "--global", "merge.tool", "Kaleidoscope"],
    None,
    &[],
    false,
  )?;
  if !status1.success() {
    anyhow::bail!("git config merge.tool failed");
  }
  let status2 = ctx.run_status(
    "git",
    &[
      "config",
      "--global",
      "mergetool.Kaleidoscope.cmd",
      r#"ksdiff --merge --output "$MERGED" --base "$BASE" -- "$LOCAL" "$REMOTE""#,
    ],
    None,
    &[],
    false,
  )?;
  if !status2.success() {
    anyhow::bail!("git config mergetool cmd failed");
  }
  let status3 = ctx.run_status(
    "git",
    &[
      "config",
      "--global",
      "mergetool.Kaleidoscope.trustExitCode",
      "true",
    ],
    None,
    &[],
    false,
  )?;
  if !status3.success() {
    anyhow::bail!("git config trustExitCode failed");
  }
  let status4 = ctx.run_status(
    "git",
    &["config", "--global", "mergetool.prompt", "false"],
    None,
    &[],
    false,
  )?;
  if !status4.success() {
    anyhow::bail!("git config mergetool.prompt failed");
  }

  ui::success("Successfully config git's mergetool");
  Ok(())
}

pub(super) fn install_git_difftool_vscode(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  utils::must_program("code")?;

  ui::step("Config git's difftool to VSCode ...");
  ui::info("Config git's difftool to VSCode");

  let status1 = ctx.run_status(
    "git",
    &["config", "--global", "diff.tool", "vscode"],
    None,
    &[],
    false,
  )?;
  if !status1.success() {
    anyhow::bail!("git config diff.tool failed");
  }
  let status2 = ctx.run_status(
    "git",
    &[
      "config",
      "--global",
      "difftool.vscode.cmd",
      r#"code --wait --diff "$LOCAL" "$REMOTE""#,
    ],
    None,
    &[],
    false,
  )?;
  if !status2.success() {
    anyhow::bail!("git config difftool.vscode.cmd failed");
  }
  let status3 = ctx.run_status(
    "git",
    &["config", "--global", "difftool.prompt", "false"],
    None,
    &[],
    false,
  )?;
  if !status3.success() {
    anyhow::bail!("git config difftool.prompt failed");
  }

  ui::success("Successfully config git's difftool");
  Ok(())
}

pub(super) fn install_git_mergetool_vscode(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  utils::must_program("code")?;

  ui::step("Config git's mergetool to VSCode ...");
  ui::info("Config git's mergetool to VSCode");

  let status1 = ctx.run_status(
    "git",
    &["config", "--global", "merge.tool", "vscode"],
    None,
    &[],
    false,
  )?;
  if !status1.success() {
    anyhow::bail!("git config merge.tool failed");
  }
  let status2 = ctx.run_status(
    "git",
    &[
      "config",
      "--global",
      "mergetool.vscode.cmd",
      r#"code --wait --merge "$REMOTE" "$LOCAL" "$BASE" "$MERGED""#,
    ],
    None,
    &[],
    false,
  )?;
  if !status2.success() {
    anyhow::bail!("git config mergetool.vscode.cmd failed");
  }
  let status3 = ctx.run_status(
    "git",
    &["config", "--global", "mergetool.prompt", "false"],
    None,
    &[],
    false,
  )?;
  if !status3.success() {
    anyhow::bail!("git config mergetool.prompt failed");
  }

  ui::success("Successfully config git's mergetool");
  Ok(())
}

pub(super) fn install_git_extras(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  ui::step("Installing git-extras ...");

  let git_extras_dir = ctx.app_path.join("git/.cache/git-extras");
  if utils::program_exists("brew") && !git_extras_dir.is_dir() {
    let status = ctx.run_status("brew", &["install", "git-extras"], None, &[], false)?;
    if !status.success() {
      anyhow::bail!("brew install git-extras failed");
    }
  } else {
    git::sync_repo(
      "https://github.com/tj/git-extras.git",
      &git_extras_dir,
      None,
      ctx,
    )?;
    let sudo = !utils::is_root();
    let status = ctx.run_status("make", &["install"], Some(&git_extras_dir), &[], sudo)?;
    if !status.success() {
      anyhow::bail!("make install failed");
    }
  }

  ui::success("Successfully installed git-extras.");
  Ok(())
}
