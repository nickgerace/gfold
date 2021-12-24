use crate::color;
use crate::error::Error;
use crate::report::{Reports, NONE};
use anyhow::Result;
use std::path::Path;

const PAD: usize = 2;

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
            color::write_status(&report.status, &report.status_as_string, status_max + PAD)?;
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

pub fn standard(reports: &Reports) -> Result<()> {
    let mut all_reports = Vec::new();
    for grouped_report in &reports.0 {
        all_reports.append(&mut grouped_report.1.clone());
    }
    all_reports.sort_by(|a, b| a.path.cmp(&b.path));
    all_reports.sort_by(|a, b| a.status_as_string.cmp(&b.status_as_string));

    let mut first = true;
    for report in all_reports {
        match first {
            true => {
                first = false;
            }
            false => println!(),
        }

        print!("📡 ");
        let full_path = Path::new(&report.parent).join(&report.path);
        color::write_group_title(&format!(
            "{} ⇒ {}",
            &report.path,
            full_path
                .to_str()
                .ok_or_else(|| Error::PathToStrConversionFailure(full_path.clone()))?
        ))?;
        color::write_status(&report.status, &report.status_as_string, PAD)?;
        println!(
            " ({})
{}
{}",
            report.branch,
            report.url,
            match &report.email {
                Some(s) => s,
                None => NONE,
            },
        );
    }
    Ok(())
}
