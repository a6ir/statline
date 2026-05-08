use std::collections::HashMap;
use std::path::{Path, PathBuf};

use plotters::prelude::*;
use polars::prelude::*;

use crate::error::Result;
use crate::profiler::ProfileReport;
use crate::profiler::utils::is_numeric;

use super::utils::{plotters_err, sanitize_filename, CHART_SIZE, TOP_K_CATEGORIES};

pub fn generate_categorical_distributions(
    df: &DataFrame,
    report: &ProfileReport,
    chart_dir: &Path,
) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();

    for profile in &report.profiles {
        let Ok(col) = df.column(&profile.name) else {
            continue;
        };
        if is_numeric(col.dtype()) {
            continue;
        }

        let casted = match col.cast(&DataType::String) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let Ok(utf8) = casted.str() else {
            continue;
        };

        let mut counts: HashMap<String, u32> = HashMap::new();
        for maybe_v in utf8 {
            if let Some(v) = maybe_v {
                *counts.entry(v.to_string()).or_insert(0) += 1;
            }
        }
        if counts.is_empty() {
            continue;
        }

        let mut top: Vec<(String, u32)> = counts.into_iter().collect();
        top.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        top.truncate(TOP_K_CATEGORIES);

        let filename = format!("{}_distribution.png", sanitize_filename(&profile.name));
        let path = chart_dir.join(filename);
        render_distribution(&path, &profile.name, &top)?;
        out.push(path);
    }

    Ok(out)
}

fn render_distribution(path: &Path, column_name: &str, categories: &[(String, u32)]) -> Result<()> {
    let y_max = categories.iter().map(|(_, c)| *c).max().unwrap_or(1).max(1);
    let root = BitMapBackend::new(path, CHART_SIZE).into_drawing_area();
    root.fill(&WHITE)
        .map_err(|e| plotters_err("failed to fill categorical background", e))?;

    let x_max = categories.len().max(1) as i32;
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Top Categories: {column_name}"), ("sans-serif", 28))
        .margin(20)
        .x_label_area_size(90)
        .y_label_area_size(50)
        .build_cartesian_2d(0i32..x_max, 0u32..y_max)
        .map_err(|e| plotters_err("failed to build categorical chart", e))?;

    chart
        .configure_mesh()
        .x_desc("Category")
        .y_desc("Count")
        .x_labels(categories.len().max(1))
        .x_label_formatter(&|idx| {
            categories
                .get(*idx as usize)
                .map(|(k, _)| truncate(k, 14))
                .unwrap_or_default()
        })
        .draw()
        .map_err(|e| plotters_err("failed to draw categorical mesh", e))?;

    chart
        .draw_series(categories.iter().enumerate().map(|(i, (_, count))| {
            Rectangle::new(
                [(i as i32, 0u32), (i as i32 + 1, *count)],
                GREEN.mix(0.6).filled(),
            )
        }))
        .map_err(|e| plotters_err("failed to draw categorical bars", e))?;

    root.present()
        .map_err(|e| plotters_err("failed to write categorical image", e))?;
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
