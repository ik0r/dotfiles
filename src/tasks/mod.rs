use std::path::{Path, PathBuf};

use regex::Regex;

use crate::ui;
use crate::utils;

mod git;
mod vim;
mod zsh;

pub struct Context {
  pub app_path: PathBuf,
  pub home_dir: PathBuf,
  pub dry_run: bool,
  pub backup: bool,
  pub jobs: usize,
}

impl Context {
  pub fn new(dry_run: bool, backup: bool, jobs: usize) -> anyhow::Result<Self> {
    Ok(Self {
      app_path: utils::app_path()?,
      home_dir: utils::home_dir()?,
      dry_run,
      backup,
      jobs: std::cmp::max(1, jobs),
    })
  }
}

pub fn print_usage() {
  eprintln!();
  eprintln!("Usage: install <task>[ taskFoo taskBar ...]");
  eprintln!();
  eprintln!("Tasks:");
  for name in task_names() {
    eprintln!("    - {}", name);
  }
  eprintln!();
}

pub fn run_task(task: &str, ctx: &Context) -> anyhow::Result<()> {
  match task {
    "editorconfig" => install_editorconfig(ctx),
    "emacs" => install_emacs(ctx),
    "emacs_spacemacs" => install_emacs_spacemacs(ctx),
    "fonts_source_code_pro" => install_fonts_source_code_pro(ctx),
    "git_alias" => install_git_alias(ctx),
    "git_config" => install_git_config(ctx),
    "git_diff_so_fancy" => install_git_diff_so_fancy(ctx),
    "git_difftool_vscode" => install_git_difftool_vscode(ctx),
    "git_mergetool_vscode" => install_git_mergetool_vscode(ctx),
    "git_difftool_kaleidoscope" => install_git_difftool_kaleidoscope(ctx),
    "git_mergetool_kaleidoscope" => install_git_mergetool_kaleidoscope(ctx),
    "git_extras" => install_git_extras(ctx),
    "homebrew" => install_homebrew(ctx),
    "tmux" => install_tmux(ctx),
    "vim_rc" => install_vim_rc(ctx),
    "vim_plugins" => install_vim_plugins(ctx),
    "vim_plugins_fcitx" => install_vim_plugins_fcitx(ctx),
    "vim_plugins_matchtag" => install_vim_plugins_matchtag(ctx),
    "vim_plugins_snippets" => install_vim_plugins_snippets(ctx),
    "vim_plugins_ycm" => anyhow::bail!("Task vim_plugins_ycm is not implemented"),
    "nvim" => install_nvim(ctx),
    "zsh_omz" => install_zsh_omz(ctx),
    "zsh_omz_cfg" => install_zsh_omz_cfg(ctx),
    "zsh_omz_plugins_git_diff_so_fancy" => install_zsh_omz_plugins_git_diff_so_fancy(ctx),
    "zsh_omz_plugins_fzf" => install_zsh_omz_plugins_fzf(ctx),
    "zsh_omz_plugins_thefuck" => install_zsh_omz_plugins_thefuck(ctx),
    "zsh_omz_plugins_zlua" => install_zsh_omz_plugins_zlua(ctx),
    "zsh_plugins_fasd" => install_zsh_plugins_fasd(ctx),
    "zsh_zim" => install_zsh_zim(ctx),
    "zsh_zim_plugins_fzf" => install_zsh_zim_plugins_fzf(ctx),
    "zsh_zim_plugins_git_diff_so_fancy" => install_zsh_zim_plugins_git_diff_so_fancy(ctx),
    "zsh_zim_plugins_omz_tmux" => install_zsh_zim_plugins_omz_tmux(ctx),
    "zsh_zim_plugins_pure" => install_zsh_zim_plugins_pure(ctx),
    "zsh_zim_plugins_zlua" => install_zsh_zim_plugins_zlua(ctx),
    _ => {
      ui::error(format!("Invalid params {}", task).as_str());
      print_usage();
      Ok(())
    }
  }
}

fn task_names() -> Vec<&'static str> {
  vec![
    "editorconfig",
    "emacs",
    "emacs_spacemacs",
    "fonts_source_code_pro",
    "git_alias",
    "git_config",
    "git_diff_so_fancy",
    "git_difftool_vscode",
    "git_mergetool_vscode",
    "git_difftool_kaleidoscope",
    "git_mergetool_kaleidoscope",
    "git_extras",
    "homebrew",
    "tmux",
    "vim_rc",
    "vim_plugins",
    "vim_plugins_fcitx",
    "vim_plugins_matchtag",
    "vim_plugins_snippets",
    "vim_plugins_ycm",
    "nvim",
    "zsh_omz",
    "zsh_omz_cfg",
    "zsh_omz_plugins_git_diff_so_fancy",
    "zsh_omz_plugins_fzf",
    "zsh_omz_plugins_thefuck",
    "zsh_omz_plugins_zlua",
    "zsh_plugins_fasd",
    "zsh_zim",
    "zsh_zim_plugins_fzf",
    "zsh_zim_plugins_git_diff_so_fancy",
    "zsh_zim_plugins_omz_tmux",
    "zsh_zim_plugins_pure",
    "zsh_zim_plugins_zlua",
  ]
}

fn install_editorconfig(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Installing editorconfig ...");
  let src = ctx.app_path.join("editorconfig/editorconfig");
  let dst = ctx.home_dir.join(".editorconfig");
  ui::info(format!("Linking {} to {}", src.display(), dst.display()).as_str());
  utils::symlink(&src, &dst, ctx.backup, ctx.dry_run)?;
  ui::tip("Maybe you should install editorconfig plugin for vim");
  ui::success("Successfully installed editorconfig.");
  Ok(())
}

fn install_emacs(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("emacs")?;
  ui::step("Installing emacs config ...");

  let repo_uri = "https://github.com/ik0r/emacs.d.git";
  let emacs_dir = ctx.home_dir.join(".emacs.d");
  if emacs_dir.is_dir() {
    let origin = git::get_remote_origin(&emacs_dir).unwrap_or_default();
    if origin != repo_uri {
      ui::tip("Your old .emacs.d is not the .emacs.d to be installed.");
      let override_it = inquire::Confirm::new("Do you want to override your old .emacs.d?")
        .with_default(true)
        .prompt()
        .unwrap();
      if override_it {
        ui::info("Remove old .emacs.d");
        if !ctx.dry_run {
          let _ = std::fs::remove_dir_all(&emacs_dir);
        }
      } else {
        ui::info("Do not override your old .emacs.d. Please remove or backup yourself.");
        return Ok(());
      }
    }
  }

  git::sync_repo(repo_uri, &emacs_dir, None, ctx)?;
  ui::success("Successfully installed emacs config.");
  Ok(())
}

fn install_emacs_spacemacs(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("emacs")?;
  ui::step("Installing spacemacs config ...");

  let repo_spacemacs_uri = "https://github.com/syl20bnr/spacemacs.git";
  let repo_config_uri = "https://github.com/ik0r/spacemacs.d.git";
  let emacs_dir = ctx.home_dir.join(".emacs.d");
  if emacs_dir.is_dir() {
    let origin = git::get_remote_origin(&emacs_dir).unwrap_or_default();
    let ok = origin == repo_spacemacs_uri || origin == format!("{}.git", repo_spacemacs_uri);
    if !ok {
      ui::tip("Your old .emacs.d is not spacemacs repo.");
      let override_it = inquire::Confirm::new("Do you want to override your old .emacs.d?")
        .with_default(true)
        .prompt()
        .unwrap();
      if override_it {
        ui::info("Remove old .emacs.d");
        if !ctx.dry_run {
          let _ = std::fs::remove_dir_all(&emacs_dir);
        }
      } else {
        ui::info("Do not override your old .emacs.d. Please remove or backup yourself.");
        return Ok(());
      }
    }
  }

  git::sync_repo(repo_spacemacs_uri, &emacs_dir, Some("develop"), ctx)?;
  let spacemacs_dir = ctx.home_dir.join(".spacemacs.d");
  git::sync_repo(repo_config_uri, &spacemacs_dir, None, ctx)?;

  ui::success("Successfully installed spacemacs and config.");
  install_fonts_source_code_pro(ctx)?;
  Ok(())
}

fn install_fonts_source_code_pro(ctx: &Context) -> anyhow::Result<()> {
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
    let name = entry.file_name().unwrap().to_string_lossy().to_string();
    let lower = name.to_lowercase();
    let ok = lower.ends_with(".ttf") || lower.ends_with(".otf") || lower.ends_with(".pcf.gz");
    if ok {
      utils::copy_file(&entry, &fonts_dir, ctx.dry_run)?;
    }
  }

  if utils::program_exists("fc-cache") {
    let _ = utils::run_status(
      "fc-cache",
      &["-f", fonts_dir.to_string_lossy().as_ref()],
      None,
      &[],
      false,
      ctx.dry_run,
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

fn install_git_alias(ctx: &Context) -> anyhow::Result<()> {
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
  let selected = inquire::Select::new("where do you select to install?", options)
    .with_starting_cursor(0)
    .prompt()
    .unwrap();

  let include_path = cache_dir.join("gitalias.txt");
  let include_path_str = include_path.to_string_lossy().to_string();

  match selected {
    "system" => {
      let sudo = !utils::is_root();
      let status = utils::run_status(
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
        ctx.dry_run,
      )?;
      if !status.success() {
        anyhow::bail!("git config --system failed");
      }
    }
    "global" => {
      let status = utils::run_status(
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
        ctx.dry_run,
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

fn install_git_config(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  ui::step("Installing gitconfig ...");

  let src = ctx.app_path.join("git/gitconfig");
  let dst = ctx.home_dir.join(".gitconfig");
  ui::info(format!("Linking {} to {}", src.display(), dst.display()).as_str());
  utils::symlink(&src, &dst, ctx.backup, ctx.dry_run)?;

  ui::info("Now config your name and email for git.");

  let user_now = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
  let user_name = inquire::Text::new(format!("What's your git username? ({})", user_now).as_str())
    .with_default(user_now.as_str())
    .prompt()
    .unwrap();
  let default_email = format!("{}@example.com", user_name);
  let user_email =
    inquire::Text::new(format!("What's your git email? ({})", default_email).as_str())
      .with_default(default_email.as_str())
      .prompt()
      .unwrap();

  let status1 = utils::run_status(
    "git",
    &["config", "--global", "user.name", user_name.as_str()],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status1.success() {
    anyhow::bail!("git config user.name failed");
  }
  let status2 = utils::run_status(
    "git",
    &["config", "--global", "user.email", user_email.as_str()],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status2.success() {
    anyhow::bail!("git config user.email failed");
  }

  ui::success("Successfully installed gitconfig.");
  Ok(())
}

fn install_git_diff_so_fancy(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  ui::step("Installing git diff-so-fancy ...");

  let cache_dir = ctx.app_path.join(".cache/diff-so-fancy");
  git::sync_repo(
    "https://github.com/so-fancy/diff-so-fancy.git",
    &cache_dir,
    None,
    ctx,
  )?;

  utils::symlink(
    &ctx.app_path.join("git/bin/git-dsf"),
    &cache_dir.join("git-dsf"),
    ctx.backup,
    ctx.dry_run,
  )?;
  utils::symlink(
    &ctx.app_path.join("git/bin/git-dsfc"),
    &cache_dir.join("git-dsfc"),
    ctx.backup,
    ctx.dry_run,
  )?;
  utils::symlink(
    &ctx.app_path.join("git/bin/git-lsp"),
    &cache_dir.join("git-lsp"),
    ctx.backup,
    ctx.dry_run,
  )?;

  ui::success("Successfully installed git diff-so-fancy.");
  ui::info(format!("Please add '{}' to your PATH.", cache_dir.display()).as_str());
  Ok(())
}

fn install_git_difftool_kaleidoscope(ctx: &Context) -> anyhow::Result<()> {
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

  let status1 = utils::run_status(
    "git",
    &["config", "--global", "diff.tool", "Kaleidoscope"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status1.success() {
    anyhow::bail!("git config diff.tool failed");
  }
  let status2 = utils::run_status(
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
    ctx.dry_run,
  )?;
  if !status2.success() {
    anyhow::bail!("git config difftool cmd failed");
  }
  let status3 = utils::run_status(
    "git",
    &["config", "--global", "difftool.prompt", "false"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status3.success() {
    anyhow::bail!("git config difftool.prompt failed");
  }

  ui::success("Successfully config git's difftool");
  Ok(())
}

fn install_git_mergetool_kaleidoscope(ctx: &Context) -> anyhow::Result<()> {
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

  let status1 = utils::run_status(
    "git",
    &["config", "--global", "merge.tool", "Kaleidoscope"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status1.success() {
    anyhow::bail!("git config merge.tool failed");
  }
  let status2 = utils::run_status(
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
    ctx.dry_run,
  )?;
  if !status2.success() {
    anyhow::bail!("git config mergetool cmd failed");
  }
  let status3 = utils::run_status(
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
    ctx.dry_run,
  )?;
  if !status3.success() {
    anyhow::bail!("git config trustExitCode failed");
  }
  let status4 = utils::run_status(
    "git",
    &["config", "--global", "mergetool.prompt", "false"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status4.success() {
    anyhow::bail!("git config mergetool.prompt failed");
  }

  ui::success("Successfully config git's mergetool");
  Ok(())
}

fn install_git_difftool_vscode(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  utils::must_program("code")?;

  ui::step("Config git's difftool to VSCode ...");
  ui::info("Config git's difftool to VSCode");

  let status1 = utils::run_status(
    "git",
    &["config", "--global", "diff.tool", "vscode"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status1.success() {
    anyhow::bail!("git config diff.tool failed");
  }
  let status2 = utils::run_status(
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
    ctx.dry_run,
  )?;
  if !status2.success() {
    anyhow::bail!("git config difftool.vscode.cmd failed");
  }
  let status3 = utils::run_status(
    "git",
    &["config", "--global", "difftool.prompt", "false"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status3.success() {
    anyhow::bail!("git config difftool.prompt failed");
  }

  ui::success("Successfully config git's difftool");
  Ok(())
}

fn install_git_mergetool_vscode(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  utils::must_program("code")?;

  ui::step("Config git's mergetool to VSCode ...");
  ui::info("Config git's mergetool to VSCode");

  let status1 = utils::run_status(
    "git",
    &["config", "--global", "merge.tool", "vscode"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status1.success() {
    anyhow::bail!("git config merge.tool failed");
  }
  let status2 = utils::run_status(
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
    ctx.dry_run,
  )?;
  if !status2.success() {
    anyhow::bail!("git config mergetool.vscode.cmd failed");
  }
  let status3 = utils::run_status(
    "git",
    &["config", "--global", "mergetool.prompt", "false"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status3.success() {
    anyhow::bail!("git config mergetool.prompt failed");
  }

  ui::success("Successfully config git's mergetool");
  Ok(())
}

fn install_git_extras(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("git")?;
  ui::step("Installing git-extras ...");

  let git_extras_dir = ctx.app_path.join("git/.cache/git-extras");
  if utils::program_exists("brew") && !git_extras_dir.is_dir() {
    let status = utils::run_status(
      "brew",
      &["install", "git-extras"],
      None,
      &[],
      false,
      ctx.dry_run,
    )?;
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
    let status = utils::run_status(
      "make",
      &["install"],
      Some(&git_extras_dir),
      &[],
      sudo,
      ctx.dry_run,
    )?;
    if !status.success() {
      anyhow::bail!("make install failed");
    }
  }

  ui::success("Successfully installed git-extras.");
  Ok(())
}

fn install_homebrew(ctx: &Context) -> anyhow::Result<()> {
  if utils::program_exists("brew") {
    ui::success("You have already installed homebrew");
    return Ok(());
  }

  utils::must_program("curl")?;

  ui::step("Installing homebrew ...");

  let url = "https://raw.githubusercontent.com/Homebrew/install/refs/heads/main/install.sh";
  let tmp = std::env::temp_dir().join("dotfiles_homebrew_install.sh");
  let tmp_str = tmp.to_string_lossy().to_string();

  let status1 = utils::run_status(
    "curl",
    &["-fsSL", url, "-o", tmp_str.as_str()],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status1.success() {
    anyhow::bail!("download homebrew install script failed");
  }

  let envs = [
    (
      "HOMEBREW_API_DOMAIN",
      "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/api",
    ),
    (
      "HOMEBREW_BOTTLE_DOMAIN",
      "https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles",
    ),
    (
      "HOMEBREW_BREW_GIT_REMOTE",
      "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git",
    ),
    (
      "HOMEBREW_CORE_GIT_REMOTE",
      "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/homebrew-core.git",
    ),
    (
      "HOMEBREW_PIP_INDEX_URL",
      "https://pypi.tuna.tsinghua.edu.cn/simple",
    ),
  ];

  let status2 = utils::run_status(
    "/bin/bash",
    &[tmp_str.as_str()],
    None,
    &envs,
    false,
    ctx.dry_run,
  )?;
  if !status2.success() {
    anyhow::bail!("homebrew install script failed");
  }

  ui::success("Successfully installed homebrew");
  Ok(())
}

fn install_tmux(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("tmux")?;
  ui::step("Installing tmux configs ...");

  let repos = vec![
    (
      "https://github.com/tmux-plugins/tpm",
      ctx.app_path.join("tmux/plugins/tpm"),
    ),
    (
      "https://github.com/tmux-plugins/tmux-sensible",
      ctx.app_path.join("tmux/plugins/tmux-sensible"),
    ),
    (
      "https://github.com/tmux-plugins/tmux-pain-control",
      ctx.app_path.join("tmux/plugins/tmux-pain-control"),
    ),
    (
      "https://github.com/christoomey/vim-tmux-navigator",
      ctx.app_path.join("tmux/plugins/vim-tmux-navigator"),
    ),
    (
      "https://github.com/tmux-plugins/tmux-prefix-highlight",
      ctx.app_path.join("tmux/plugins/tmux-prefix-highlight"),
    ),
    (
      "https://github.com/tmux-plugins/tmux-copycat",
      ctx.app_path.join("tmux/plugins/tmux-copycat"),
    ),
    (
      "https://github.com/tmux-plugins/tmux-yank",
      ctx.app_path.join("tmux/plugins/tmux-yank"),
    ),
    (
      "https://github.com/NHDaly/tmux-better-mouse-mode",
      ctx.app_path.join("tmux/plugins/tmux-better-mouse-mode"),
    ),
  ];
  git::sync_repo_batch(repos, ctx)?;

  if utils::is_mac() && !utils::program_exists("reattach-to-user-namespace") {
    if utils::program_exists("brew") {
      let status = utils::run_status(
        "brew",
        &["install", "reattach-to-user-namespace"],
        None,
        &[],
        false,
        ctx.dry_run,
      )?;
      if !status.success() {
        anyhow::bail!("brew install reattach-to-user-namespace failed");
      }
    } else {
      ui::tip("Maybe you should install reattach-to-user-namespace for vim in tmux");
    }
  }

  utils::symlink(
    &ctx.app_path.join("tmux"),
    &ctx.home_dir.join(".tmux"),
    ctx.backup,
    ctx.dry_run,
  )?;
  utils::symlink(
    &ctx.app_path.join("tmux/tmux.conf"),
    &ctx.home_dir.join(".tmux.conf"),
    ctx.backup,
    ctx.dry_run,
  )?;

  ui::success("Please run tmux and use prefix-U to update tmux plugins or reload your tmux.conf");
  Ok(())
}

fn install_vim_rc(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("vim")?;
  ui::step("Installing vimrc ...");
  utils::symlink(
    &ctx.app_path.join("vim"),
    &ctx.home_dir.join(".vim"),
    ctx.backup,
    ctx.dry_run,
  )?;
  utils::symlink(
    &ctx.app_path.join("vim/vimrc"),
    &ctx.home_dir.join(".vimrc"),
    ctx.backup,
    ctx.dry_run,
  )?;
  ui::success("Successfully installed vimrc.");
  ui::success("You can add your own configs to ~/.vimrc.local, vim will source them automatically");
  Ok(())
}

fn install_vim_plugins(ctx: &Context) -> anyhow::Result<()> {
  if !ctx.home_dir.join(".vimrc").exists() {
    anyhow::bail!("You should complete vim_rc task first");
  }

  ui::step("Initializing vim-plug");
  git::sync_repo(
    "https://github.com/junegunn/vim-plug.git",
    &ctx.app_path.join("vim/autoload"),
    None,
    ctx,
  )?;
  utils::symlink(
    &ctx.app_path.join("vim/vimrc.plugins"),
    &ctx.home_dir.join(".vimrc.plugins"),
    ctx.backup,
    ctx.dry_run,
  )?;

  let status = utils::run_status(
    "vim",
    &["+PlugInstall", "+qall"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status.success() {
    anyhow::bail!("vim PlugInstall failed");
  }

  if !utils::program_exists("ag") {
    ui::tip("Maybe you can take full use of this by installing one of (ag)~");
  }

  ui::success(
    "You can add your own plugins to ~/.vimrc.plugins.local , vim will source them automatically",
  );
  install_fonts_source_code_pro(ctx)?;
  ui::tip(
    "In order to use powerline symbols with airline in vim, please set your terminal to use the font *Source Code Pro*",
  );
  Ok(())
}

fn must_vimrc_plugins(ctx: &Context) -> anyhow::Result<()> {
  if !ctx.home_dir.join(".vimrc.plugins").exists() {
    anyhow::bail!("You should complete vim_plugins task first");
  }
  Ok(())
}

fn install_vim_plugins_fcitx(ctx: &Context) -> anyhow::Result<()> {
  must_vimrc_plugins(ctx)?;
  ui::step("installing fcitx support plugin for vim ...");

  if utils::is_mac() {
    let im = std::env::var("FCITX_IM").unwrap_or_default();
    if im.is_empty() {
      anyhow::bail!("You must set FCITX_IM to use fcitx-vim-osx plugin");
    }
    git::sync_repo(
      "https://github.com/CodeFalling/fcitx-remote-for-osx.git",
      &ctx.app_path.join("vim/.cache/fcitx-remote-for-osx"),
      Some("binary"),
      ctx,
    )?;
    utils::symlink(
      &ctx.app_path.join(format!(
        "vim/.cache/fcitx-remote-for-osx/fcitx-remote-{}",
        im
      )),
      Path::new("/usr/local/bin/fcitx-remote"),
      ctx.backup,
      ctx.dry_run,
    )?;
  }

  vim::append_dotvim_group("fcitx", ctx)?;
  let status = utils::run_status(
    "vim",
    &["+PlugInstall", "+qall"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status.success() {
    anyhow::bail!("vim PlugInstall failed");
  }
  ui::success("Successfully installed fcitx support plugin.");
  Ok(())
}

fn install_vim_plugins_matchtag(ctx: &Context) -> anyhow::Result<()> {
  must_vimrc_plugins(ctx)?;
  utils::must_program("python")?;
  ui::step("Installing vim MatchTagAlways plugin ...");

  vim::ensure_neovim_python_support(ctx)?;
  vim::append_dotvim_group("matchtag", ctx)?;

  let status = utils::run_status(
    "vim",
    &["+PlugInstall", "+qall"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status.success() {
    anyhow::bail!("vim PlugInstall failed");
  }
  ui::success("Successfully installed MatchTagAlways plugins.");
  Ok(())
}

fn install_vim_plugins_snippets(ctx: &Context) -> anyhow::Result<()> {
  must_vimrc_plugins(ctx)?;
  utils::must_program("python")?;
  ui::step("Installing vim snippets plugin ...");

  vim::ensure_neovim_python_support(ctx)?;
  vim::append_dotvim_group("snippets", ctx)?;

  let status = utils::run_status(
    "vim",
    &["+PlugInstall", "+qall"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status.success() {
    anyhow::bail!("vim PlugInstall failed");
  }
  ui::success("Successfully installed vim-snippets plugins.");
  Ok(())
}

fn install_nvim(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("nvim")?;
  ui::step("Installing nvim ...");

  utils::symlink(
    &ctx.app_path.join("nvim"),
    &ctx.home_dir.join(".config/nvim"),
    ctx.backup,
    ctx.dry_run,
  )?;

  let status = utils::run_status(
    "nvim",
    &["--headless", "+Lazy! sync", "+qa"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status.success() {
    anyhow::bail!("nvim lazy sync failed");
  }

  ui::success("Successfully installed nvim");
  Ok(())
}

fn install_zsh_omz(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("zsh")?;
  ui::step("Installing oh-my-zsh for zsh ...");

  git::sync_repo(
    "https://github.com/ohmyzsh/ohmyzsh.git",
    &ctx.app_path.join("zsh/.cache/ohmyzsh"),
    None,
    ctx,
  )?;
  git::sync_repo(
    "https://github.com/zsh-users/zsh-syntax-highlighting.git",
    &ctx
      .app_path
      .join("zsh/.cache/ohmyzsh/custom/plugins/zsh-syntax-highlighting"),
    None,
    ctx,
  )?;
  git::sync_repo(
    "https://github.com/tarruda/zsh-autosuggestions.git",
    &ctx
      .app_path
      .join("zsh/.cache/ohmyzsh/custom/plugins/zsh-autosuggestions"),
    None,
    ctx,
  )?;

  utils::symlink(
    &ctx.app_path.join("zsh/.cache/ohmyzsh"),
    &ctx.home_dir.join(".oh-my-zsh"),
    ctx.backup,
    ctx.dry_run,
  )?;
  utils::symlink(
    &ctx.app_path.join("zsh/omz/.zshrc"),
    &ctx.home_dir.join(".zshrc"),
    ctx.backup,
    ctx.dry_run,
  )?;

  zsh::ensure_default_shell_is_zsh(ctx)?;

  ui::success("Successfully installed zsh and oh-my-zsh.");
  ui::tip("You can add your own configs to ~/.zshrc.local , zsh will source them automatically");
  ui::success("Please open a new zsh terminal to make configs go into effect.");
  Ok(())
}

fn install_zsh_omz_cfg(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("zsh")?;
  ui::step("Installing omz configs ...");
  utils::symlink(
    &ctx.app_path.join("zsh/omz/.zshrc.local"),
    &ctx.home_dir.join(".zshrc.local"),
    ctx.backup,
    ctx.dry_run,
  )?;
  ui::success("Successfully installed omz configs");
  ui::success("Please open a new zsh terminal to make configs go into effect.");
  Ok(())
}

fn install_zsh_plugins_fasd(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Installing fasd plugin for zsh ...");
  let dir = ctx.app_path.join("zsh/.cache/fasd");
  git::sync_repo("https://github.com/clvv/fasd.git", &dir, None, ctx)?;
  let sudo = !utils::is_root();
  let status = utils::run_status("make", &["install"], Some(&dir), &[], sudo, ctx.dry_run)?;
  if !status.success() {
    anyhow::bail!("make install failed");
  }
  ui::success("Successfully installed fasd plugin.");
  ui::success("Please open a new zsh terminal to make configs go into effect.");
  Ok(())
}

fn install_zsh_omz_plugins_git_diff_so_fancy(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Install git diff-so-fancy plugin for oh-my-zsh ...");
  let cache_dir = ctx.app_path.join(".cache/diff-so-fancy");
  git::sync_repo(
    "https://github.com/so-fancy/diff-so-fancy.git",
    &cache_dir,
    None,
    ctx,
  )?;

  utils::symlink(
    &ctx.app_path.join("git/bin/git-dsf"),
    &cache_dir.join("git-dsf"),
    ctx.backup,
    ctx.dry_run,
  )?;
  utils::symlink(
    &ctx.app_path.join("git/bin/git-dsfc"),
    &cache_dir.join("git-dsfc"),
    ctx.backup,
    ctx.dry_run,
  )?;
  utils::symlink(
    &ctx.app_path.join("git/bin/git-lsp"),
    &cache_dir.join("git-lsp"),
    ctx.backup,
    ctx.dry_run,
  )?;

  utils::symlink(
    &cache_dir,
    &ctx
      .app_path
      .join("zsh/.cache/ohmyzsh/custom/plugins/diff-so-fancy"),
    ctx.backup,
    ctx.dry_run,
  )?;

  ui::success("Successfully installed git diff-so-fancy for oh-my-zsh.");
  Ok(())
}

fn install_zsh_omz_plugins_fzf(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Installing fzf plugin for oh-my-zsh ...");
  let dir = ctx.app_path.join("zsh/.cache/fzf");
  git::sync_repo("https://github.com/junegunn/fzf.git", &dir, None, ctx)?;

  let status = utils::run_status(
    dir.join("install").to_string_lossy().as_ref(),
    &["--bin"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status.success() {
    anyhow::bail!("fzf install --bin failed");
  }

  let zshenv = ctx.home_dir.join(".zshenv");
  let pattern = Regex::new(
    format!(
      r#"(?im)^[ \t]*export[ \t]*FZF_BASE="{}"[ \t]*$"#,
      regex::escape(dir.to_string_lossy().as_ref())
    )
    .as_str(),
  )?;
  utils::append_line_if_missing(
    &zshenv,
    &pattern,
    format!("export FZF_BASE=\"{}\"", dir.to_string_lossy()).as_str(),
    ctx.dry_run,
  )?;

  ui::success("Successfully installed fzf plugin.");
  Ok(())
}

fn install_zsh_omz_plugins_thefuck(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Installing thefuck plugin for oh-my-zsh ...");

  zsh::must_python_pip(ctx)?;

  let pip = if utils::program_exists("pip3") {
    "pip3"
  } else if utils::program_exists("pip2") {
    "pip2"
  } else {
    "pip"
  };

  let status = utils::run_status(
    pip,
    &["install", "--user", "--upgrade", "thefuck"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status.success() {
    anyhow::bail!("pip install thefuck failed");
  }

  let plugin_dir = ctx
    .app_path
    .join("zsh/.cache/ohmyzsh/custom/plugins/thefuck");
  if !ctx.dry_run {
    utils::ensure_dir(&plugin_dir)?;
    std::fs::write(
      plugin_dir.join("thefuck.plugin.zsh"),
      r#"eval "$(thefuck --alias)""#,
    )?;
  }

  ui::success("Successfully installed thefuck plugin.");
  ui::success("Please open a new zsh terminal to make configs go into effect.");
  Ok(())
}

fn install_zsh_omz_plugins_zlua(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("zsh")?;
  utils::must_program("lua")?;
  ui::step("Installing z.lua for oh-my-zsh");

  let dir = ctx.app_path.join("zsh/.cache/z.lua");
  git::sync_repo("https://github.com/skywind3000/z.lua.git", &dir, None, ctx)?;
  utils::symlink(
    &dir,
    &ctx.app_path.join("zsh/.cache/ohmyzsh/custom/plugins/z.lua"),
    ctx.backup,
    ctx.dry_run,
  )?;

  ui::success("Successfully installed z.lua for oh-my-zsh.");
  Ok(())
}

fn install_zsh_zim(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("zsh")?;
  utils::must_program("curl")?;
  ui::step("Installing zim for zsh ...");

  let zim_home = ctx.app_path.join("zsh/.cache/zimfw");
  let zim_home_str = zim_home.to_string_lossy().to_string();

  let url = "https://raw.githubusercontent.com/zimfw/install/master/install.zsh";
  let tmp = std::env::temp_dir().join("dotfiles_zimfw_install.zsh");
  let tmp_str = tmp.to_string_lossy().to_string();

  let status1 = utils::run_status(
    "curl",
    &["-fsSL", url, "-o", tmp_str.as_str()],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status1.success() {
    anyhow::bail!("download zim install script failed");
  }

  let status2 = utils::run_status(
    "zsh",
    &[tmp_str.as_str()],
    None,
    &[("ZIM_HOME", zim_home_str.as_str())],
    false,
    ctx.dry_run,
  )?;
  if !status2.success() {
    anyhow::bail!("zim install script failed");
  }

  ui::success("Successfully installed zim.");
  Ok(())
}

fn install_zsh_zim_plugins_fzf(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("zsh")?;
  utils::must_program("curl")?;

  ui::step("Install fzf plugin for zim ...");

  let dir = ctx.app_path.join("zsh/.cache/fzf");
  git::sync_repo("https://github.com/junegunn/fzf.git", &dir, None, ctx)?;

  let status = utils::run_status(
    dir.join("install").to_string_lossy().as_ref(),
    &["--bin"],
    None,
    &[],
    false,
    ctx.dry_run,
  )?;
  if !status.success() {
    anyhow::bail!("fzf install --bin failed");
  }

  let zshenv = ctx.home_dir.join(".zshenv");
  let pattern = Regex::new(
    format!(
      r#"(?im)^[ \t]*export[ \t]*FZF_BASE="{}"[ \t]*$"#,
      regex::escape(dir.to_string_lossy().as_ref())
    )
    .as_str(),
  )?;
  utils::append_line_if_missing(
    &zshenv,
    &pattern,
    format!("export FZF_BASE=\"{}\"", dir.to_string_lossy()).as_str(),
    ctx.dry_run,
  )?;

  let zimrc = ctx.home_dir.join(".zimrc");
  let zmodule_line = "zmodule ohmyzsh/ohmyzsh --source plugins/fzf/fzf.plugin.zsh";
  let zim_pattern = Regex::new(
    r"(?im)^[ \t]*zmodule[ \t]+ohmyzsh/ohmyzsh[ \t]+--source[ \t]+plugins/fzf/fzf\.plugin\.zsh[ \t]*$",
  )?;
  utils::append_line_if_missing(&zimrc, &zim_pattern, zmodule_line, ctx.dry_run)?;

  ui::success("Successfully installed fzf for zim.");
  Ok(())
}

fn install_zsh_zim_plugins_git_diff_so_fancy(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Install git diff-so-fancy plugin for zim ...");
  let cache_dir = ctx.app_path.join(".cache/diff-so-fancy");
  git::sync_repo(
    "https://github.com/so-fancy/diff-so-fancy.git",
    &cache_dir,
    None,
    ctx,
  )?;

  utils::symlink(
    &ctx.app_path.join("git/bin/git-dsf"),
    &cache_dir.join("git-dsf"),
    ctx.backup,
    ctx.dry_run,
  )?;
  utils::symlink(
    &ctx.app_path.join("git/bin/git-dsfc"),
    &cache_dir.join("git-dsfc"),
    ctx.backup,
    ctx.dry_run,
  )?;
  utils::symlink(
    &ctx.app_path.join("git/bin/git-lsp"),
    &cache_dir.join("git-lsp"),
    ctx.backup,
    ctx.dry_run,
  )?;

  utils::symlink(
    &cache_dir,
    &ctx.app_path.join("zsh/.cache/zimfw/modules/diff-so-fancy"),
    ctx.backup,
    ctx.dry_run,
  )?;

  let zimrc = ctx.home_dir.join(".zimrc");
  let line = "zmodule so-fancy/diff-so-fancy";
  let pattern = Regex::new(r"(?im)^[ \t]*zmodule[ \t]+so-fancy/diff-so-fancy[ \t]*$")?;
  utils::append_line_if_missing(&zimrc, &pattern, line, ctx.dry_run)?;

  ui::success("Successfully installed git diff-so-fancy for zim.");
  Ok(())
}

fn install_zsh_zim_plugins_omz_tmux(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Install tmux plugin for zim ...");
  utils::must_program("zsh")?;
  utils::must_program("tmux")?;

  let ohmyzsh_dir = ctx.app_path.join("zsh/.cache/ohmyzsh");
  git::sync_repo(
    "https://github.com/ohmyzsh/ohmyzsh.git",
    &ohmyzsh_dir,
    None,
    ctx,
  )?;

  utils::symlink(
    &ohmyzsh_dir,
    &ctx.app_path.join("zsh/.cache/zimfw/modules/ohmyzsh"),
    ctx.backup,
    ctx.dry_run,
  )?;

  let zimrc = ctx.home_dir.join(".zimrc");
  let line = "zmodule ohmyzsh/ohmyzsh --source plugins/tmux/tmux.plugin.zsh";
  let pattern = Regex::new(
    r"(?im)^[ \t]*zmodule[ \t]+ohmyzsh/ohmyzsh[ \t]+--source[ \t]+plugins/tmux/tmux\.plugin\.zsh[ \t]*$",
  )?;
  utils::append_line_if_missing(&zimrc, &pattern, line, ctx.dry_run)?;

  ui::success("Successfully installed tmux for zim.");
  Ok(())
}

fn install_zsh_zim_plugins_pure(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Install pure theme for zim ...");
  let dir = ctx.app_path.join("zsh/.cache/pure");
  git::sync_repo(
    "https://github.com/sindresorhus/pure.git",
    &dir,
    Some("main"),
    ctx,
  )?;

  utils::symlink(
    &dir,
    &ctx.app_path.join("zsh/.cache/zimfw/modules/pure"),
    ctx.backup,
    ctx.dry_run,
  )?;

  let zimrc = ctx.home_dir.join(".zimrc");
  let line = "zmodule sindresorhus/pure --source async.zsh --source pure.zsh";
  let pattern = Regex::new(
    r"(?im)^[ \t]*zmodule[ \t]+sindresorhus/pure[ \t]+--source[ \t]+async\.zsh[ \t]+--source[ \t]+pure\.zsh[ \t]*$",
  )?;
  utils::append_line_if_missing(&zimrc, &pattern, line, ctx.dry_run)?;

  ui::success("Successfully install pure theme for zim.");
  Ok(())
}

fn install_zsh_zim_plugins_zlua(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("zsh")?;
  utils::must_program("lua")?;
  ui::step("Install z.lua for zim ...");

  let dir = ctx.app_path.join("zsh/.cache/z.lua");
  git::sync_repo("https://github.com/skywind3000/z.lua.git", &dir, None, ctx)?;

  utils::symlink(
    &dir,
    &ctx.app_path.join("zsh/.cache/zimfw/modules/z.lua"),
    ctx.backup,
    ctx.dry_run,
  )?;

  let zimrc = ctx.home_dir.join(".zimrc");
  let line = "zmodule skywind3000/z.lua";
  let pattern = Regex::new(r"(?im)^[ \t]*zmodule[ \t]+skywind3000/z\.lua[ \t]*$")?;
  utils::append_line_if_missing(&zimrc, &pattern, line, ctx.dry_run)?;

  ui::success("Successfully install z.lua for zim.");
  Ok(())
}
