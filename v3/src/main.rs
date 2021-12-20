use std::path::Path;

mod color;
mod display;
mod error;
mod report;
mod status;
mod target_gen;

use report::Reports;
use target_gen::Targets;

fn main() {
    let foo = Targets::new(Path::new("/Users/nick/src").to_path_buf()).unwrap();
    let bar = Reports::new(foo).unwrap();
    display::classic(&bar).unwrap();
}
