use std::path::{Path, PathBuf};

use crate::ui;
use crate::utils;

mod emacs;
mod fonts;
mod git;
mod git_tasks;
mod vim;
mod vim_tasks;
mod zsh;
mod zsh_tasks;

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

  pub fn run_status(
    &self,
    program: &str,
    args: &[&str],
    cwd: Option<&Path>,
    envs: &[(&str, &str)],
    sudo: bool,
  ) -> anyhow::Result<std::process::ExitStatus> {
    if self.dry_run {
      let cmd = format_cmd(program, args, sudo);
      let mut meta = Vec::new();
      if let Some(cwd) = cwd {
        meta.push(format!("cwd={}", cwd.display()));
      }
      if !envs.is_empty() {
        let keys = envs.iter().map(|(k, _)| *k).collect::<Vec<_>>().join(",");
        meta.push(format!("env={}", keys));
      }
      if meta.is_empty() {
        ui::info(format!("dry-run: {}", cmd).as_str());
      } else {
        ui::info(format!("dry-run: {} ({})", cmd, meta.join(" ")).as_str());
      }
    }
    utils::run_status(program, args, cwd, envs, sudo, self.dry_run)
  }

  pub fn run_status_quiet(
    &self,
    program: &str,
    args: &[&str],
    cwd: Option<&Path>,
    envs: &[(&str, &str)],
    sudo: bool,
  ) -> anyhow::Result<std::process::ExitStatus> {
    utils::run_status(program, args, cwd, envs, sudo, self.dry_run)
  }

  pub fn symlink(&self, src: &Path, dst: &Path) -> anyhow::Result<()> {
    if self.dry_run {
      ui::info(format!("dry-run: ln -s {} {}", src.display(), dst.display()).as_str());
    }
    utils::symlink(src, dst, self.backup, self.dry_run)
  }
}

fn format_cmd(program: &str, args: &[&str], sudo: bool) -> String {
  let mut parts = Vec::new();
  if sudo {
    parts.push("sudo".to_string());
  }
  parts.push(shell_quote(program));
  for arg in args {
    parts.push(shell_quote(arg));
  }
  parts.join(" ")
}

fn shell_quote(s: &str) -> String {
  let safe = s
    .chars()
    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '/' | ':' | '@' | '='));
  if safe {
    return s.to_string();
  }
  format!("'{}'", s.replace('\'', r#"'\''"#))
}

pub fn print_usage() {
  eprintln!();
  eprintln!("Usage: install <task>[ taskFoo taskBar ...]");
  eprintln!();
  eprintln!("Tasks:");
  for task in TASKS {
    eprintln!("    - {}", task.name);
  }
  eprintln!();
}

pub fn run_task(task: &str, ctx: &Context) -> anyhow::Result<()> {
  let Some(task_def) = TASKS.iter().find(|t| t.name == task) else {
    ui::error(format!("Invalid params {}", task).as_str());
    print_usage();
    return Ok(());
  };
  (task_def.run)(ctx)
}

struct TaskDef {
  name: &'static str,
  run: fn(&Context) -> anyhow::Result<()>,
}

static TASKS: &[TaskDef] = &[
  TaskDef {
    name: "editorconfig",
    run: install_editorconfig,
  },
  TaskDef {
    name: "emacs",
    run: emacs::install_emacs,
  },
  TaskDef {
    name: "emacs_spacemacs",
    run: emacs::install_emacs_spacemacs,
  },
  TaskDef {
    name: "fonts_source_code_pro",
    run: fonts::install_fonts_source_code_pro,
  },
  TaskDef {
    name: "git_alias",
    run: git_tasks::install_git_alias,
  },
  TaskDef {
    name: "git_config",
    run: git_tasks::install_git_config,
  },
  TaskDef {
    name: "git_diff_so_fancy",
    run: git_tasks::install_git_diff_so_fancy,
  },
  TaskDef {
    name: "git_difftool_vscode",
    run: git_tasks::install_git_difftool_vscode,
  },
  TaskDef {
    name: "git_mergetool_vscode",
    run: git_tasks::install_git_mergetool_vscode,
  },
  TaskDef {
    name: "git_difftool_kaleidoscope",
    run: git_tasks::install_git_difftool_kaleidoscope,
  },
  TaskDef {
    name: "git_mergetool_kaleidoscope",
    run: git_tasks::install_git_mergetool_kaleidoscope,
  },
  TaskDef {
    name: "git_extras",
    run: git_tasks::install_git_extras,
  },
  TaskDef {
    name: "homebrew",
    run: install_homebrew,
  },
  TaskDef {
    name: "tmux",
    run: install_tmux,
  },
  TaskDef {
    name: "vim_rc",
    run: vim_tasks::install_vim_rc,
  },
  TaskDef {
    name: "vim_plugins",
    run: vim_tasks::install_vim_plugins,
  },
  TaskDef {
    name: "vim_plugins_fcitx",
    run: vim_tasks::install_vim_plugins_fcitx,
  },
  TaskDef {
    name: "vim_plugins_matchtag",
    run: vim_tasks::install_vim_plugins_matchtag,
  },
  TaskDef {
    name: "vim_plugins_snippets",
    run: vim_tasks::install_vim_plugins_snippets,
  },
  TaskDef {
    name: "vim_plugins_ycm",
    run: install_vim_plugins_ycm,
  },
  TaskDef {
    name: "nvim",
    run: vim_tasks::install_nvim,
  },
  TaskDef {
    name: "zsh_omz",
    run: zsh_tasks::install_zsh_omz,
  },
  TaskDef {
    name: "zsh_omz_cfg",
    run: zsh_tasks::install_zsh_omz_cfg,
  },
  TaskDef {
    name: "zsh_omz_plugins_git_diff_so_fancy",
    run: zsh_tasks::install_zsh_omz_plugins_git_diff_so_fancy,
  },
  TaskDef {
    name: "zsh_omz_plugins_fzf",
    run: zsh_tasks::install_zsh_omz_plugins_fzf,
  },
  TaskDef {
    name: "zsh_omz_plugins_thefuck",
    run: zsh_tasks::install_zsh_omz_plugins_thefuck,
  },
  TaskDef {
    name: "zsh_omz_plugins_zlua",
    run: zsh_tasks::install_zsh_omz_plugins_zlua,
  },
  TaskDef {
    name: "zsh_plugins_fasd",
    run: zsh_tasks::install_zsh_plugins_fasd,
  },
  TaskDef {
    name: "zsh_zim",
    run: zsh_tasks::install_zsh_zim,
  },
  TaskDef {
    name: "zsh_zim_plugins_fzf",
    run: zsh_tasks::install_zsh_zim_plugins_fzf,
  },
  TaskDef {
    name: "zsh_zim_plugins_git_diff_so_fancy",
    run: zsh_tasks::install_zsh_zim_plugins_git_diff_so_fancy,
  },
  TaskDef {
    name: "zsh_zim_plugins_omz_tmux",
    run: zsh_tasks::install_zsh_zim_plugins_omz_tmux,
  },
  TaskDef {
    name: "zsh_zim_plugins_pure",
    run: zsh_tasks::install_zsh_zim_plugins_pure,
  },
  TaskDef {
    name: "zsh_zim_plugins_zlua",
    run: zsh_tasks::install_zsh_zim_plugins_zlua,
  },
];

fn install_editorconfig(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Installing editorconfig ...");
  let src = ctx.app_path.join("editorconfig/editorconfig");
  let dst = ctx.home_dir.join(".editorconfig");
  ui::info(format!("Linking {} to {}", src.display(), dst.display()).as_str());
  ctx.symlink(&src, &dst)?;
  ui::tip("Maybe you should install editorconfig plugin for vim");
  ui::success("Successfully installed editorconfig.");
  Ok(())
}

fn install_vim_plugins_ycm(_ctx: &Context) -> anyhow::Result<()> {
  anyhow::bail!("Task vim_plugins_ycm is not implemented")
}

fn install_homebrew(ctx: &Context) -> anyhow::Result<()> {
  if utils::program_exists("brew") {
    ui::success("You have already installed homebrew");
    return Ok(());
  }

  utils::must_program("curl")?;

  ui::step("Installing homebrew ...");

  let url = "https://raw.githubusercontent.com/Homebrew/install/refs/heads/main/install.sh";
  let tmp = utils::TempFile::new("dotfiles_homebrew_install", ".sh", !ctx.dry_run)?;
  let tmp_str = tmp.path().to_string_lossy().to_string();

  let status1 = ctx.run_status(
    "curl",
    &["-fsSL", url, "-o", tmp_str.as_str()],
    None,
    &[],
    false,
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

  let status2 = ctx.run_status("/bin/bash", &[tmp_str.as_str()], None, &envs, false)?;
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
      let status = ctx.run_status(
        "brew",
        &["install", "reattach-to-user-namespace"],
        None,
        &[],
        false,
      )?;
      if !status.success() {
        anyhow::bail!("brew install reattach-to-user-namespace failed");
      }
    } else {
      ui::tip("Maybe you should install reattach-to-user-namespace for vim in tmux");
    }
  }

  ctx.symlink(&ctx.app_path.join("tmux"), &ctx.home_dir.join(".tmux"))?;
  ctx.symlink(
    &ctx.app_path.join("tmux/tmux.conf"),
    &ctx.home_dir.join(".tmux.conf"),
  )?;

  ui::success("Please run tmux and use prefix-U to update tmux plugins or reload your tmux.conf");
  Ok(())
}
