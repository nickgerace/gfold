use crate::task::TaskRunner;
use crate::TaskHarness;
use crate::TaskResult;

pub struct RunBuild;

impl TaskRunner for RunBuild {
    fn run(harness: &mut TaskHarness) -> TaskResult<()> {
        harness.cargo("build --all-targets", None)?;
        Ok(())
    }
}
