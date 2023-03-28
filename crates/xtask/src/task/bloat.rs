use crate::TaskHarness;
use crate::TaskResult;
use crate::TaskRunner;

pub struct RunBloat;

impl TaskRunner for RunBloat {
    fn run(harness: &mut TaskHarness) -> TaskResult<()> {
        harness.cargo("bloat --release", None)?;
        harness.cargo("bloat --release --crates", None)?;
        Ok(())
    }
}
