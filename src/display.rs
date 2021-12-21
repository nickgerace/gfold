use std::path::Path;

use crate::color;
use crate::report::Reports;
use anyhow::Result;

pub const PAD: usize = 2;

pub fn classic(reports: &Reports) -> Result<()> {
    let length = reports.0.keys().len();
    let mut first = true;
    for group in &reports.0 {
        // FIXME: make group title matching less cumbersome.
        if length > 1 {
            match first {
                true => {
                    first = false;
                }
                false => println!(),
            }
            color::write_group_title(group.0)?;
        }

        let mut path_max = 0;
        let mut branch_max = 0;
        let mut status_max = 0;
        for report in group.1 {
            if report.path.len() > path_max {
                path_max = report.path.len();
            }
            let status_length = report.status_as_string.len();
            if status_length > status_max {
                status_max = status_length;
            }
            if report.branch.len() > branch_max {
                branch_max = report.branch.len();
            }
        }

        let mut reports = group.1.clone();
        reports.sort_by(|a, b| a.path.cmp(&b.path));
        reports.sort_by(|a, b| a.status_as_string.cmp(&b.status_as_string));

        for report in reports {
            print!("{:<path_width$}", report.path, path_width = path_max + PAD);
            color::write_status(&report.status, status_max + PAD)?;
            println!(
                "{:<branch_width$}{}",
                report.branch,
                report.url,
                branch_width = branch_max + PAD
            );
        }
    }
    Ok(())
}

// TODO: finish this functionality.
pub fn modern(reports: &Reports) -> Result<()> {
    for i in &reports.0 {
        for i in i.1 {
            println!("{:?}", Path::new(&i.parent).join(&i.path));
        }
    }
    Ok(())
}
