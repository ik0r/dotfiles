use std::path::{Path, PathBuf};

use regex::Regex;

use crate::ui;
use crate::utils;

mod emacs;
mod fonts;
mod git;
mod git_tasks;
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
      ui::info(format!("dry-run: {}", cmd).as_str());
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
    run: install_vim_rc,
  },
  TaskDef {
    name: "vim_plugins",
    run: install_vim_plugins,
  },
  TaskDef {
    name: "vim_plugins_fcitx",
    run: install_vim_plugins_fcitx,
  },
  TaskDef {
    name: "vim_plugins_matchtag",
    run: install_vim_plugins_matchtag,
  },
  TaskDef {
    name: "vim_plugins_snippets",
    run: install_vim_plugins_snippets,
  },
  TaskDef {
    name: "vim_plugins_ycm",
    run: install_vim_plugins_ycm,
  },
  TaskDef {
    name: "nvim",
    run: install_nvim,
  },
  TaskDef {
    name: "zsh_omz",
    run: install_zsh_omz,
  },
  TaskDef {
    name: "zsh_omz_cfg",
    run: install_zsh_omz_cfg,
  },
  TaskDef {
    name: "zsh_omz_plugins_git_diff_so_fancy",
    run: install_zsh_omz_plugins_git_diff_so_fancy,
  },
  TaskDef {
    name: "zsh_omz_plugins_fzf",
    run: install_zsh_omz_plugins_fzf,
  },
  TaskDef {
    name: "zsh_omz_plugins_thefuck",
    run: install_zsh_omz_plugins_thefuck,
  },
  TaskDef {
    name: "zsh_omz_plugins_zlua",
    run: install_zsh_omz_plugins_zlua,
  },
  TaskDef {
    name: "zsh_plugins_fasd",
    run: install_zsh_plugins_fasd,
  },
  TaskDef {
    name: "zsh_zim",
    run: install_zsh_zim,
  },
  TaskDef {
    name: "zsh_zim_plugins_fzf",
    run: install_zsh_zim_plugins_fzf,
  },
  TaskDef {
    name: "zsh_zim_plugins_git_diff_so_fancy",
    run: install_zsh_zim_plugins_git_diff_so_fancy,
  },
  TaskDef {
    name: "zsh_zim_plugins_omz_tmux",
    run: install_zsh_zim_plugins_omz_tmux,
  },
  TaskDef {
    name: "zsh_zim_plugins_pure",
    run: install_zsh_zim_plugins_pure,
  },
  TaskDef {
    name: "zsh_zim_plugins_zlua",
    run: install_zsh_zim_plugins_zlua,
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

fn install_vim_rc(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("vim")?;
  ui::step("Installing vimrc ...");
  ctx.symlink(&ctx.app_path.join("vim"), &ctx.home_dir.join(".vim"))?;
  ctx.symlink(
    &ctx.app_path.join("vim/vimrc"),
    &ctx.home_dir.join(".vimrc"),
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
  ctx.symlink(
    &ctx.app_path.join("vim/vimrc.plugins"),
    &ctx.home_dir.join(".vimrc.plugins"),
  )?;

  let status = ctx.run_status("vim", &["+PlugInstall", "+qall"], None, &[], false)?;
  if !status.success() {
    anyhow::bail!("vim PlugInstall failed");
  }

  if !utils::program_exists("ag") {
    ui::tip("Maybe you can take full use of this by installing one of (ag)~");
  }

  ui::success(
    "You can add your own plugins to ~/.vimrc.plugins.local , vim will source them automatically",
  );
  fonts::install_fonts_source_code_pro(ctx)?;
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
    ctx.symlink(
      &ctx.app_path.join(format!(
        "vim/.cache/fcitx-remote-for-osx/fcitx-remote-{}",
        im
      )),
      Path::new("/usr/local/bin/fcitx-remote"),
    )?;
  }

  vim::append_dotvim_group("fcitx", ctx)?;
  let status = ctx.run_status("vim", &["+PlugInstall", "+qall"], None, &[], false)?;
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

  let status = ctx.run_status("vim", &["+PlugInstall", "+qall"], None, &[], false)?;
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

  let status = ctx.run_status("vim", &["+PlugInstall", "+qall"], None, &[], false)?;
  if !status.success() {
    anyhow::bail!("vim PlugInstall failed");
  }
  ui::success("Successfully installed vim-snippets plugins.");
  Ok(())
}

fn install_nvim(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("nvim")?;
  ui::step("Installing nvim ...");

  ctx.symlink(
    &ctx.app_path.join("nvim"),
    &ctx.home_dir.join(".config/nvim"),
  )?;

  let status = ctx.run_status(
    "nvim",
    &["--headless", "+Lazy! sync", "+qa"],
    None,
    &[],
    false,
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

  ctx.symlink(
    &ctx.app_path.join("zsh/.cache/ohmyzsh"),
    &ctx.home_dir.join(".oh-my-zsh"),
  )?;
  ctx.symlink(
    &ctx.app_path.join("zsh/omz/.zshrc"),
    &ctx.home_dir.join(".zshrc"),
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
  ctx.symlink(
    &ctx.app_path.join("zsh/omz/.zshrc.local"),
    &ctx.home_dir.join(".zshrc.local"),
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
  let status = ctx.run_status("make", &["install"], Some(&dir), &[], sudo)?;
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

  ctx.symlink(
    &cache_dir,
    &ctx
      .app_path
      .join("zsh/.cache/ohmyzsh/custom/plugins/diff-so-fancy"),
  )?;

  ui::success("Successfully installed git diff-so-fancy for oh-my-zsh.");
  Ok(())
}

fn install_zsh_omz_plugins_fzf(ctx: &Context) -> anyhow::Result<()> {
  ui::step("Installing fzf plugin for oh-my-zsh ...");
  let dir = ctx.app_path.join("zsh/.cache/fzf");
  git::sync_repo("https://github.com/junegunn/fzf.git", &dir, None, ctx)?;

  let status = ctx.run_status(
    dir.join("install").to_string_lossy().as_ref(),
    &["--bin"],
    None,
    &[],
    false,
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

  let status = ctx.run_status(
    pip,
    &["install", "--user", "--upgrade", "thefuck"],
    None,
    &[],
    false,
  )?;
  if !status.success() {
    anyhow::bail!("pip install thefuck failed");
  }

  let plugin_dir = ctx
    .app_path
    .join("zsh/.cache/ohmyzsh/custom/plugins/thefuck");
  if !ctx.dry_run {
    utils::ensure_dir(&plugin_dir)?;
    utils::atomic_write_string(
      plugin_dir.join("thefuck.plugin.zsh").as_path(),
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
  ctx.symlink(
    &dir,
    &ctx.app_path.join("zsh/.cache/ohmyzsh/custom/plugins/z.lua"),
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
  let tmp = utils::TempFile::new("dotfiles_zimfw_install", ".zsh", !ctx.dry_run)?;
  let tmp_str = tmp.path().to_string_lossy().to_string();

  let status1 = ctx.run_status(
    "curl",
    &["-fsSL", url, "-o", tmp_str.as_str()],
    None,
    &[],
    false,
  )?;
  if !status1.success() {
    anyhow::bail!("download zim install script failed");
  }

  let status2 = ctx.run_status(
    "zsh",
    &[tmp_str.as_str()],
    None,
    &[("ZIM_HOME", zim_home_str.as_str())],
    false,
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

  let status = ctx.run_status(
    dir.join("install").to_string_lossy().as_ref(),
    &["--bin"],
    None,
    &[],
    false,
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

  ctx.symlink(
    &cache_dir,
    &ctx.app_path.join("zsh/.cache/zimfw/modules/diff-so-fancy"),
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

  ctx.symlink(
    &ohmyzsh_dir,
    &ctx.app_path.join("zsh/.cache/zimfw/modules/ohmyzsh"),
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

  ctx.symlink(&dir, &ctx.app_path.join("zsh/.cache/zimfw/modules/pure"))?;

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

  ctx.symlink(&dir, &ctx.app_path.join("zsh/.cache/zimfw/modules/z.lua"))?;

  let zimrc = ctx.home_dir.join(".zimrc");
  let line = "zmodule skywind3000/z.lua";
  let pattern = Regex::new(r"(?im)^[ \t]*zmodule[ \t]+skywind3000/z\.lua[ \t]*$")?;
  utils::append_line_if_missing(&zimrc, &pattern, line, ctx.dry_run)?;

  ui::success("Successfully install z.lua for zim.");
  Ok(())
}
