use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn home_dir() -> anyhow::Result<PathBuf> {
  let home = env::var("HOME")?;
  Ok(PathBuf::from(home))
}

pub fn app_path() -> anyhow::Result<PathBuf> {
  if let Ok(exe) = env::current_exe() {
    for dir in exe.ancestors() {
      if dir.join("install.sh").exists() {
        return Ok(dir.to_path_buf());
      }
    }
  }

  let mut current_dir = env::current_dir()?;
  loop {
    if current_dir.join("install.sh").exists() {
      return Ok(current_dir);
    }
    let parent = match current_dir.parent() {
      Some(parent) => parent.to_path_buf(),
      None => return Ok(env::current_dir()?),
    };
    current_dir = parent;
  }
}

pub fn is_mac() -> bool {
  env::consts::OS == "macos"
}

pub fn is_linux() -> bool {
  env::consts::OS == "linux"
}

pub fn program_exists(program: &str) -> bool {
  let path = match env::var_os("PATH") {
    Some(path) => path,
    None => return false,
  };

  for dir in env::split_paths(&path) {
    let full = dir.join(program);
    if is_executable(&full) {
      return true;
    }
  }

  false
}

fn is_executable(path: &Path) -> bool {
  if !path.is_file() {
    return false;
  }
  #[cfg(unix)]
  {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(meta) = fs::metadata(path) {
      return meta.permissions().mode() & 0o111 != 0;
    }
  }
  #[cfg(not(unix))]
  {
    let _ = path;
  }
  false
}

pub fn must_program(program: &str) -> anyhow::Result<()> {
  if !program_exists(program) {
    anyhow::bail!("You must have *{}* installed!", program);
  }
  Ok(())
}

pub fn must_file(path: &Path) -> anyhow::Result<()> {
  if !path.exists() {
    anyhow::bail!("You must have file *{}*", path.display());
  }
  Ok(())
}

pub fn ensure_dir(path: &Path) -> anyhow::Result<()> {
  fs::create_dir_all(path)?;
  Ok(())
}

pub fn run_output(
  program: &str,
  args: &[&str],
  cwd: Option<&Path>,
  envs: &[(&str, &str)],
  sudo: bool,
) -> anyhow::Result<Output> {
  let mut cmd = if sudo {
    let mut cmd = Command::new("sudo");
    cmd.arg(program);
    cmd
  } else {
    Command::new(program)
  };

  cmd.args(args);
  if let Some(cwd) = cwd {
    cmd.current_dir(cwd);
  }
  for (k, v) in envs {
    cmd.env(k, v);
  }

  let output = cmd.output()?;
  Ok(output)
}

pub fn run_status(
  program: &str,
  args: &[&str],
  cwd: Option<&Path>,
  envs: &[(&str, &str)],
  sudo: bool,
  dry_run: bool,
) -> anyhow::Result<ExitStatus> {
  if dry_run {
    return Ok(fake_exit_status());
  }

  let mut cmd = if sudo {
    let mut cmd = Command::new("sudo");
    cmd.arg(program);
    cmd
  } else {
    Command::new(program)
  };

  cmd.args(args);
  if let Some(cwd) = cwd {
    cmd.current_dir(cwd);
  }
  for (k, v) in envs {
    cmd.env(k, v);
  }

  let status = cmd
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()?;
  Ok(status)
}

fn fake_exit_status() -> ExitStatus {
  #[cfg(unix)]
  {
    use std::os::unix::process::ExitStatusExt;
    return ExitStatus::from_raw(0);
  }
  #[cfg(not(unix))]
  {
    panic!("dry-run not supported on this platform");
  }
}

pub fn is_root() -> bool {
  let output = Command::new("id").arg("-u").output();
  let Ok(output) = output else {
    return false;
  };
  if !output.status.success() {
    return false;
  }
  let s = String::from_utf8_lossy(&output.stdout);
  s.trim() == "0"
}

pub fn symlink(src: &Path, dst: &Path, backup: bool, dry_run: bool) -> anyhow::Result<()> {
  if dry_run {
    return Ok(());
  }

  if let Some(parent) = dst.parent() {
    ensure_dir(parent)?;
  }

  if dst.exists() || dst.is_symlink() {
    if let Ok(target) = fs::read_link(dst) {
      if target == src {
        return Ok(());
      }
    }

    if backup {
      let backup_path = backup_path(dst)?;
      if let Some(parent) = backup_path.parent() {
        ensure_dir(parent)?;
      }
      fs::rename(dst, &backup_path)?;
    } else if dst.is_dir() && !dst.is_symlink() {
      fs::remove_dir_all(dst)?;
    } else {
      let _ = fs::remove_file(dst);
      let _ = fs::remove_dir_all(dst);
    }
  }

  #[cfg(unix)]
  {
    use std::os::unix::fs::symlink;
    symlink(src, dst)?;
  }

  Ok(())
}

fn backup_path(dst: &Path) -> anyhow::Result<PathBuf> {
  let home = home_dir()?;
  let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
  let rel = dst
    .strip_prefix(&home)
    .unwrap_or(dst)
    .iter()
    .collect::<PathBuf>();
  Ok(home.join(".dotfiles_backup").join(ts.to_string()).join(rel))
}

pub fn read_to_string_if_exists(path: &Path) -> anyhow::Result<String> {
  if !path.exists() {
    return Ok(String::new());
  }
  Ok(fs::read_to_string(path)?)
}

pub fn append_line_if_missing(
  path: &Path,
  pattern: &regex::Regex,
  line: &str,
  dry_run: bool,
) -> anyhow::Result<()> {
  let content = read_to_string_if_exists(path)?;
  if pattern.is_match(content.as_str()) {
    return Ok(());
  }

  if dry_run {
    return Ok(());
  }

  if let Some(parent) = path.parent() {
    ensure_dir(parent)?;
  }

  let mut new_content = content;
  if !new_content.ends_with('\n') && !new_content.is_empty() {
    new_content.push('\n');
  }
  new_content.push_str(line);
  new_content.push('\n');
  fs::write(path, new_content)?;
  Ok(())
}

pub fn basename(path: &Path) -> Option<&OsStr> {
  path.file_name()
}

pub fn copy_file(src: &Path, dst_dir: &Path, dry_run: bool) -> anyhow::Result<()> {
  if dry_run {
    return Ok(());
  }
  ensure_dir(dst_dir)?;
  let name =
    basename(src).ok_or_else(|| io::Error::new(io::ErrorKind::Other, "invalid filename"))?;
  let dst = dst_dir.join(name);
  fs::copy(src, dst)?;
  Ok(())
}
