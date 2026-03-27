use regex::Regex;

use crate::tasks::Context;
use crate::tasks::git;
use crate::tasks::zsh;
use crate::ui;
use crate::utils;

pub(super) fn install_zsh_omz(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_omz_cfg(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_plugins_fasd(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_omz_plugins_git_diff_so_fancy(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_omz_plugins_fzf(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_omz_plugins_thefuck(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_omz_plugins_zlua(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_zim(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_zim_plugins_fzf(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_zim_plugins_git_diff_so_fancy(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_zim_plugins_omz_tmux(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_zim_plugins_pure(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_zsh_zim_plugins_zlua(ctx: &Context) -> anyhow::Result<()> {
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
