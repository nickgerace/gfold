use crate::TaskHarness;
use crate::TaskResult;
use crate::TaskRunner;

pub struct RunBuildRelease;

impl TaskRunner for RunBuildRelease {
    fn run(harness: &mut TaskHarness) -> TaskResult<()> {
        harness.cargo("build --release", None)?;
        Ok(())
    }
}
