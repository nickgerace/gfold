use crate::{Task, TaskResult};
use crate::{TaskHarness, TaskRunner};

pub struct RunScan;

impl TaskRunner for RunScan {
    fn run(harness: &mut TaskHarness) -> TaskResult<()> {
        harness.task(Task::Prepare)?;
        harness.cargo("+nightly udeps", None)?;
        harness.cargo("audit", None)?;
        Ok(())
    }
}
