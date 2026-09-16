mod collect;
mod config;
mod process;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use collect::collect_targets;
use config::Config;
use log::{debug, info, warn};
use process::process_targets;
use rayon::ThreadPoolBuilder;
use std::io;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::thread;

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[arg(long)]
    sequential: bool,
    #[arg(long)]
    parallel_collect_threads: Option<NonZeroUsize>,
    #[arg(long)]
    dry_run: bool,
    #[arg(short = 'i', long)]
    ignore_config_file: bool,
    #[arg(long)]
    generate_man_page: bool,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    debug!("initialized logger");
    if cli.generate_man_page {
        debug!("generating man page");
        print_man_page()?;
        return Ok(());
    }
    let config = Config::resolve(
        cli.path,
        cli.sequential,
        cli.parallel_collect_threads,
        cli.ignore_config_file,
    )?;
    debug!("resolved config: {config:?}");
    if cli.dry_run {
        debug!("printing resolved config");
        print_config(&config)?;
        return Ok(());
    }
    run(config)
}

fn print_config(config: &Config) -> Result<()> {
    print!("{}", toml::to_string(config)?);
    Ok(())
}

fn print_man_page() -> Result<()> {
    let man = clap_mangen::Man::new(Cli::command());
    let mut stdout = io::stdout().lock();
    man.render(&mut stdout)?;
    Ok(())
}

fn run(config: Config) -> Result<()> {
    info!("collecting jj workspaces under {}", config.path.display());
    let targets = if config.sequential {
        debug!("collecting workspaces sequentially");
        collect_targets(config.path, true, true)?
    } else if let Some(parallel_collect_threads) = config.parallel_collect_threads {
        debug!("collecting workspaces with up to {parallel_collect_threads} threads");
        collect_targets_parallel(config.path, parallel_collect_threads)?
    } else {
        debug!("collecting workspaces with the global Rayon thread pool");
        collect_targets(config.path, false, true)?
    };
    info!("found {} jj workspaces", targets.len());
    process_targets(targets)?;
    info!("finished checking jj workspaces");
    Ok(())
}

fn collect_targets_parallel(
    path: PathBuf,
    parallel_collect_threads: NonZeroUsize,
) -> Result<Vec<PathBuf>> {
    let parallel_collect_threads = usable_parallel_collect_threads(parallel_collect_threads);
    Ok(ThreadPoolBuilder::new()
        .num_threads(parallel_collect_threads)
        .build()?
        .install(|| collect_targets(path, false, true))?)
}

fn usable_parallel_collect_threads(parallel_collect_threads: NonZeroUsize) -> usize {
    let requested_threads = parallel_collect_threads.get();
    let Ok(available_threads) = thread::available_parallelism() else {
        return requested_threads;
    };
    let available_threads = available_threads.get();
    if requested_threads > available_threads {
        warn!(
            "requested {requested_threads} parallel collect threads, but only {available_threads} logical threads are available; using {available_threads}"
        );
    }

    effective_parallel_collect_threads(requested_threads, available_threads)
}

fn effective_parallel_collect_threads(requested_threads: usize, available_threads: usize) -> usize {
    requested_threads.min(available_threads)
}

#[cfg(test)]
mod tests {
    use clap::Parser as _;
    use log::LevelFilter;

    use super::{Cli, effective_parallel_collect_threads};

    #[test]
    fn verbosity_flags_set_log_level() {
        let default = Cli::try_parse_from(["jjfold"]).unwrap();
        let verbose = Cli::try_parse_from(["jjfold", "-v"]).unwrap();
        let trace = Cli::try_parse_from(["jjfold", "-vv"]).unwrap();
        let quiet = Cli::try_parse_from(["jjfold", "-q"]).unwrap();

        assert_eq!(default.verbose.log_level_filter(), LevelFilter::Info);
        assert_eq!(verbose.verbose.log_level_filter(), LevelFilter::Debug);
        assert_eq!(trace.verbose.log_level_filter(), LevelFilter::Trace);
        assert_eq!(quiet.verbose.log_level_filter(), LevelFilter::Warn);
    }

    #[test]
    fn keeps_requested_parallel_collect_threads_within_available_threads() {
        assert_eq!(effective_parallel_collect_threads(2, 4), 2);
    }

    #[test]
    fn caps_parallel_collect_threads_at_available_threads() {
        assert_eq!(effective_parallel_collect_threads(8, 4), 4);
    }
}
