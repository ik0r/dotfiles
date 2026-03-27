use clap::Parser;
use inquire::InquireError;

mod prompt;
mod tasks;
mod ui;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
  #[arg(trailing_var_arg = true)]
  pub tasks: Vec<String>,

  #[arg(long)]
  pub list: bool,

  #[arg(long)]
  pub dry_run: bool,

  #[arg(long)]
  pub no_backup: bool,

  #[arg(long, default_value_t = 4)]
  pub jobs: usize,
}

fn main() {
  if let Err(err) = run() {
    if is_user_cancelled(&err) {
      ui::tip("Cancelled.");
      std::process::exit(130);
    }
    ui::error(format!("{}", err).as_str());
    std::process::exit(1);
  }
}

fn run() -> anyhow::Result<()> {
  let cli = Cli::parse();

  if cli.list {
    tasks::print_usage();
    return Ok(());
  }

  if cli.tasks.is_empty() {
    tasks::print_usage();
    return Ok(());
  }

  let ctx = tasks::Context::new(cli.dry_run, !cli.no_backup, cli.jobs)?;

  for task in &cli.tasks {
    tasks::run_task(task.as_str(), &ctx)?;
  }

  Ok(())
}

fn is_user_cancelled(err: &anyhow::Error) -> bool {
  let Some(inquire_err) = err.downcast_ref::<InquireError>() else {
    return false;
  };
  matches!(
    inquire_err,
    InquireError::OperationCanceled | InquireError::OperationInterrupted
  )
}
