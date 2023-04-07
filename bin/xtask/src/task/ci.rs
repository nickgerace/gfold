use crate::TaskResult;
use crate::{TaskHarness, TaskRunner};

pub struct RunCi;

impl TaskRunner for RunCi {
    fn run(harness: &mut TaskHarness) -> TaskResult<()> {
        harness.cargo("fmt --all -- --check", None)?;
        harness.cargo("check --all-targets --all-features --workspace", None)?;
        harness.cargo(
            "clippy --all-targets --all-features --no-deps --workspace -- -D warnings",
            None,
        )?;
        harness.cargo("doc --all --no-deps", Some(("RUSTDOCFLAGS", "-Dwarnings")))?;
        harness.cargo("test --all-targets --workspace", None)?;
        harness.cargo("build --locked --all-targets", None)?;
        Ok(())
    }
}
