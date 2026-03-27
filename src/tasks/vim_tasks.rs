use std::path::Path;

use crate::tasks::Context;
use crate::tasks::fonts;
use crate::tasks::git;
use crate::tasks::vim;
use crate::ui;
use crate::utils;

pub(super) fn install_vim_rc(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_vim_plugins(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_vim_plugins_fcitx(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_vim_plugins_matchtag(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_vim_plugins_snippets(ctx: &Context) -> anyhow::Result<()> {
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

pub(super) fn install_nvim(ctx: &Context) -> anyhow::Result<()> {
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

fn must_vimrc_plugins(ctx: &Context) -> anyhow::Result<()> {
  if !ctx.home_dir.join(".vimrc.plugins").exists() {
    anyhow::bail!("You should complete vim_plugins task first");
  }
  Ok(())
}
