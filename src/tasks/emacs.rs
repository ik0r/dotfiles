use crate::prompt;
use crate::tasks::Context;
use crate::tasks::fonts;
use crate::tasks::git;
use crate::ui;
use crate::utils;

pub(super) fn install_emacs(ctx: &Context) -> anyhow::Result<()> {
  utils::must_program("emacs")?;
  ui::step("Installing emacs config ...");

  let repo_uri = "https://github.com/ik0r/emacs.d.git";
  let emacs_dir = ctx.home_dir.join(".emacs.d");
  if emacs_dir.is_dir() {
    let origin = git::get_remote_origin(&emacs_dir).unwrap_or_default();
    if origin != repo_uri {
      ui::tip("Your old .emacs.d is not the .emacs.d to be installed.");
      let override_it = prompt::confirm("Do you want to override your old .emacs.d?", true)?;
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

pub(super) fn install_emacs_spacemacs(ctx: &Context) -> anyhow::Result<()> {
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
      let override_it = prompt::confirm("Do you want to override your old .emacs.d?", true)?;
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
  fonts::install_fonts_source_code_pro(ctx)?;
  Ok(())
}
