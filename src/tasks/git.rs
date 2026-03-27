use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::tasks::Context;
use crate::ui;
use crate::utils;

pub fn get_remote_origin(repo_path: &Path) -> Option<String> {
  let output = utils::run_output(
    "git",
    &["remote", "get-url", "origin"],
    Some(repo_path),
    &[],
    false,
  )
  .ok()?;
  if !output.status.success() {
    return None;
  }
  let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
  if s.is_empty() {
    return None;
  }
  Some(s)
}

pub fn sync_repo(
  repo_uri: &str,
  repo_path: &Path,
  repo_branch: Option<&str>,
  ctx: &Context,
) -> anyhow::Result<()> {
  utils::must_program("git")?;

  let repo_name = repo_uri
    .strip_prefix("https://github.com/")
    .unwrap_or(repo_uri)
    .to_string();

  let git_dir = repo_path.join(".git");
  if !git_dir.is_dir() {
    ui::info(format!("Cloning {} ...", repo_name).as_str());
    if !ctx.dry_run {
      utils::ensure_dir(repo_path)?;
    }

    let mut args = vec!["clone", "--depth", "1"];
    let mut branch_args = Vec::new();
    if let Some(branch) = repo_branch {
      if !branch.is_empty() {
        branch_args.push("--branch");
        branch_args.push(branch);
      }
    }
    args.extend(branch_args);
    args.push(repo_uri);
    let repo_path_str = repo_path.to_string_lossy().to_string();
    args.push(repo_path_str.as_str());

    let status = utils::run_status("git", &args, None, &[], false, ctx.dry_run)?;
    if !status.success() {
      anyhow::bail!("git clone failed: {}", repo_uri);
    }
    ui::success(format!("Successfully cloned {}.", repo_name).as_str());
  } else {
    ui::info(format!("Updating {} ...", repo_name).as_str());
    let branch = current_branch(repo_path).unwrap_or_else(|| "HEAD".to_string());
    let status = utils::run_status(
      "git",
      &["pull", "origin", branch.as_str()],
      Some(repo_path),
      &[],
      false,
      ctx.dry_run,
    )?;
    if !status.success() {
      anyhow::bail!("git pull failed: {}", repo_uri);
    }
    ui::success(format!("Successfully updated {}.", repo_name).as_str());
  }

  if repo_path.join(".gitmodules").is_file() {
    ui::info(format!("Updating {} submodules ...", repo_name).as_str());
    let status = utils::run_status(
      "git",
      &["submodule", "update", "--init", "--recursive"],
      Some(repo_path),
      &[],
      false,
      ctx.dry_run,
    )?;
    if !status.success() {
      anyhow::bail!("git submodule update failed: {}", repo_uri);
    }
    ui::success(format!("Successfully updated {} submodules.", repo_name).as_str());
  }

  Ok(())
}

fn current_branch(repo_path: &Path) -> Option<String> {
  let output = utils::run_output(
    "git",
    &["branch", "--show-current"],
    Some(repo_path),
    &[],
    false,
  )
  .ok()?;
  if !output.status.success() {
    return None;
  }
  let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
  if s.is_empty() {
    return None;
  }
  Some(s)
}

pub fn sync_repo_batch(repos: Vec<(&'static str, PathBuf)>, ctx: &Context) -> anyhow::Result<()> {
  if ctx.jobs <= 1 || repos.len() <= 1 {
    for (uri, path) in repos {
      sync_repo(uri, &path, None, ctx)?;
    }
    return Ok(());
  }

  let queue = Arc::new(Mutex::new(repos.into_iter().collect::<Vec<_>>()));
  let errs = Arc::new(Mutex::new(Vec::<anyhow::Error>::new()));

  let mut handles = Vec::new();
  for _ in 0..ctx.jobs {
    let queue = queue.clone();
    let errs = errs.clone();
    let ctx = Context {
      app_path: ctx.app_path.clone(),
      home_dir: ctx.home_dir.clone(),
      dry_run: ctx.dry_run,
      backup: ctx.backup,
      jobs: 1,
    };
    handles.push(thread::spawn(move || {
      loop {
        let item = {
          let mut q = queue.lock().unwrap();
          q.pop()
        };
        let Some((uri, path)) = item else {
          break;
        };
        if let Err(err) = sync_repo(uri, &path, None, &ctx) {
          errs.lock().unwrap().push(err);
        }
      }
    }));
  }

  for h in handles {
    let _ = h.join();
  }

  let errs = errs.lock().unwrap();
  if !errs.is_empty() {
    anyhow::bail!("sync repo batch failed: {} errors", errs.len());
  }
  Ok(())
}
