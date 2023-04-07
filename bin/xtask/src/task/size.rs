use std::path::PathBuf;
use std::{fs, io};

use crate::task::CRATE;
use crate::TaskResult;
use crate::{Task, TaskError};
use crate::{TaskHarness, TaskRunner};

pub struct RunSize;

impl TaskRunner for RunSize {
    fn run(harness: &mut TaskHarness) -> TaskResult<()> {
        harness.task(Task::BuildRelease)?;

        let binary = harness.root.join("target").join("release").join(CRATE);
        harness.print_binary_size(&binary, false)?;

        // Check if the binary is currently installed to compare release build sizes. If it is not,
        // we can skip this check.
        let home = dirs::home_dir().ok_or(TaskError::HomeDirectoryNotFound)?;
        let installed = home.join(".cargo").join("bin").join(CRATE);
        harness.print_binary_size(&installed, true)?;
        Ok(())
    }
}

impl TaskHarness {
    fn print_binary_size(&self, binary: &PathBuf, skip_if_not_found: bool) -> TaskResult<()> {
        let size = match fs::metadata(binary) {
            Ok(metadata) => metadata.len() as f64 / 1048576.0,
            Err(e) if e.kind() == io::ErrorKind::NotFound => match skip_if_not_found {
                true => {
                    self.stdout(format!("skipping, binary not found: {}", binary.display()))?;
                    return Ok(());
                }
                false => {
                    self.stderr(format!("error binary not found: {}", binary.display()))?;
                    return Err(e.into());
                }
            },
            Err(e) => return Err(e.into()),
        };

        // Divisor used to perform human readable size conversion.
        // 1048576.0 = 1024.0 * 1024.0
        self.stdout(format!("{:.3} MB - {}", size, binary.display()))?;
        Ok(())
    }
}
