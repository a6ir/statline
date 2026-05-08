use std::path::{Path, PathBuf};

use plotters::prelude::*;

use crate::error::Result;
use crate::profiler::ProfileReport;

use super::utils::{plotters_err, CHART_SIZE};

pub fn generate_missing_chart(report: &ProfileReport, chart_dir: &Path) -> Result<Option<PathBuf>> {
    if report.profiles.is_empty() {
        return Ok(None);
    }

    let data: Vec<(&str, f64)> = report
        .profiles
        .iter()
        .map(|p| (p.name.as_str(), p.null_pct))
        .collect();

    let path = chart_dir.join("nulls.png");
    render_missing_chart(&path, &data)?;
    Ok(Some(path))
}

fn render_missing_chart(path: &Path, data: &[(&str, f64)]) -> Result<()> {
    let root = BitMapBackend::new(path, CHART_SIZE).into_drawing_area();
    root.fill(&WHITE)
        .map_err(|e| plotters_err("failed to fill missing chart background", e))?;

    let x_max = data.len().max(1) as i32;
    let mut chart = ChartBuilder::on(&root)
        .caption("Missing Values (%)", ("sans-serif", 28))
        .margin(20)
        .x_label_area_size(100)
        .y_label_area_size(55)
        .build_cartesian_2d(0i32..x_max, 0f64..100f64)
        .map_err(|e| plotters_err("failed to build missing chart", e))?;

    chart
        .configure_mesh()
        .x_desc("Column")
        .y_desc("Null %")
        .x_labels(data.len().max(1))
        .x_label_formatter(&|idx| {
            data.get(*idx as usize)
                .map(|(name, _)| truncate(name, 14))
                .unwrap_or_default()
        })
        .draw()
        .map_err(|e| plotters_err("failed to draw missing chart mesh", e))?;

    chart
        .draw_series(data.iter().enumerate().map(|(idx, (_, null_pct))| {
            Rectangle::new(
                [(idx as i32, 0.0), (idx as i32 + 1, *null_pct)],
                RED.mix(0.6).filled(),
            )
        }))
        .map_err(|e| plotters_err("failed to draw missing chart bars", e))?;

    root.present()
        .map_err(|e| plotters_err("failed to write missing chart image", e))?;
    Ok(())
}

fn truncate(input: &str, max_len: usize) -> String {
    if input.chars().count() <= max_len {
        return input.to_string();
    }
    let mut out = input
        .chars()
        .take(max_len.saturating_sub(3))
        .collect::<String>();
    out.push_str("...");
    out
}
