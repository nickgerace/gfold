use crate::TaskResult;
use crate::{TaskHarness, TaskRunner};

pub struct RunPrepare;

impl TaskRunner for RunPrepare {
    fn run(harness: &mut TaskHarness) -> TaskResult<()> {
        harness.cargo("update", None)?;
        harness.cargo("fmt", None)?;
        harness.cargo("check --all-targets --all-features --workspace", None)?;
        harness.cargo("fix --edition-idioms --allow-dirty --allow-staged", None)?;
        harness.cargo("clippy --all-features --all-targets --workspace --no-deps --fix --allow-dirty --allow-staged", None)?;
        Ok(())
    }
}
