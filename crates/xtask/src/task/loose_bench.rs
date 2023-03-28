//! This is the loose bench for gfold. Why use a loose bench over a precise one? Most of gfold
//! development happens with a mobile processor, which is prone to performance fluctuations over
//! background I/O operations, battery life and temperature. Moreover, the "real world" use case of
//! gfold is via CLI and not a library. Thus, this loose bench executes gfold as a CLI similarly
//! to real world use. This benchmark is neither precise nor "scientific" and is purely designed to
//! give a high level performance overview.

// TODO(nick): add consistency deviation. We should see how far each result strays from the average.
// We want gfold to perform consistently as well as quickly.

use std::cmp::Ordering;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use crate::{Task, TaskError, TaskResult};
use crate::{TaskHarness, TaskRunner};

pub struct RunLooseBench;

const RUNS: usize = 100;

impl TaskRunner for RunLooseBench {
    fn run(harness: &mut TaskHarness) -> TaskResult<()> {
        let global_instant = Instant::now();

        harness.task(Task::BuildRelease)?;

        let release = harness.root.join("target").join("release").join("gfold");
        let home = dirs::home_dir().ok_or(TaskError::HomeDirectoryNotFound)?;
        let installed = home.join(".cargo").join("bin").join("gfold");
        let target = harness
            .root
            .parent()
            .ok_or(TaskError::RepositoryDoesNotHaveParentDirectory)?;

        // Loosely bench with the parent directory of the repository as the target.
        let mut release_durations = Vec::new();
        let mut installed_durations = Vec::new();
        let mut first = true;

        // Add "1" to total runs to ensure caching does not skew results. We need one
        // "warm up" run.
        let runs_with_warm_up = RUNS + 1;
        for run in 0..runs_with_warm_up {
            println!("run {run} of {RUNS}");

            // Alternate at the halfway point. Though, it is doubtful that this would actually
            // change anything. Only calculate the average after the first run to avoid caching
            // skewing results.
            if run > RUNS / 2 {
                let (old_duration, new_duration) =
                    harness.loose_bench(&installed, &release, target)?;
                if first {
                    first = false;
                } else {
                    release_durations.push(new_duration);
                    installed_durations.push(old_duration);
                }
            } else {
                let (new_duration, old_duration) =
                    harness.loose_bench(&release, &installed, target)?;
                if first {
                    first = false;
                } else {
                    release_durations.push(new_duration);
                    installed_durations.push(old_duration);
                }
            }
        }

        let release_average = harness.average_duration(&release_durations);
        let installed_average = harness.average_duration(&installed_durations);
        let (difference, label) = match release_average.cmp(&installed_average) {
            Ordering::Greater => (
                release_average - installed_average,
                "installed was faster than release",
            ),
            Ordering::Less => (
                installed_average - release_average,
                "release was faster than installed",
            ),
            Ordering::Equal => (Duration::from_millis(0), "tied"),
        };

        println!(
            "
target: {}

  release     {release_average:.2?}  {}
  installed   {installed_average:.2?}  {}
  difference  {difference:.2?}  ({label})

loose bench duration: {:.2?}",
            target.display(),
            &release.display(),
            &installed.display(),
            global_instant.elapsed()
        );
        Ok(())
    }
}

impl TaskHarness {
    fn loose_bench(
        &self,
        first: &Path,
        second: &Path,
        target: &Path,
    ) -> TaskResult<(Duration, Duration)> {
        let first_duration = self.execute_gfold(first, target)?;
        let second_duration = self.execute_gfold(second, target)?;

        let (first_text, second_text) = match first_duration.cmp(&second_duration) {
            Ordering::Greater => ("slower", "faster"),
            Ordering::Less => ("faster", "slower"),
            Ordering::Equal => ("tied  ", "tied  "),
        };

        println!("  {first_text}  {first_duration:.2?}  {}", first.display(),);
        println!(
            "  {second_text}  {second_duration:.2?}  {}",
            second.display(),
        );
        Ok((first_duration, second_duration))
    }

    fn execute_gfold(&self, binary: &Path, target: &Path) -> TaskResult<Duration> {
        let start = Instant::now();
        let output = Command::new(binary).arg("-i").arg(target).output()?;
        match output.status.success() {
            true => Ok(start.elapsed()),
            false => Err(TaskError::UnsuccessfulCommandDuringLooseBench(output)),
        }
    }

    fn average_duration(&self, durations: &[Duration]) -> Duration {
        let sum = durations.iter().sum::<Duration>();
        let count = durations.len() as f32;
        sum.div_f32(count)
    }
}
