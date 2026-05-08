use std::path::{Path, PathBuf};

use plotters::prelude::*;
use polars::prelude::*;

use crate::error::Result;
use crate::profiler::ProfileReport;
use crate::profiler::utils::is_numeric;

use super::utils::{plotters_err, sanitize_filename, CHART_SIZE};

pub fn generate_histograms(df: &DataFrame, report: &ProfileReport, chart_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();

    for profile in &report.profiles {
        let Ok(series) = df.column(&profile.name) else {
            continue;
        };
        if !is_numeric(series.dtype()) {
            continue;
        }
        let casted = match series.cast(&DataType::Float64) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let Ok(ca) = casted.f64() else {
            continue;
        };
        let values: Vec<f64> = ca.into_iter().flatten().filter(|v| v.is_finite()).collect();
        if values.is_empty() {
            continue;
        }

        let filename = format!("{}_histogram.png", sanitize_filename(&profile.name));
        let path = chart_dir.join(filename);
        render_histogram(&path, &profile.name, &values)?;
        out.push(path);
    }

    Ok(out)
}

fn render_histogram(path: &Path, column_name: &str, values: &[f64]) -> Result<()> {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !min.is_finite() || !max.is_finite() {
        return Ok(());
    }

    let bins = 20usize.min(values.len().max(1));
    let width = if (max - min).abs() < f64::EPSILON {
        1.0
    } else {
        (max - min) / bins as f64
    };

    let mut counts = vec![0u32; bins];
    for v in values {
        let mut idx = if width > 0.0 {
            ((v - min) / width).floor() as isize
        } else {
            0
        };
        if idx < 0 {
            idx = 0;
        }
        if idx as usize >= bins {
            idx = bins as isize - 1;
        }
        counts[idx as usize] += 1;
    }

    let y_max = counts.iter().copied().max().unwrap_or(1).max(1);
    let x_hi = if (max - min).abs() < f64::EPSILON {
        min + 1.0
    } else {
        max
    };

    let root = BitMapBackend::new(path, CHART_SIZE).into_drawing_area();
    root.fill(&WHITE)
        .map_err(|e| plotters_err("failed to fill histogram background", e))?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Histogram: {column_name}"), ("sans-serif", 28))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(min..x_hi, 0u32..y_max)
        .map_err(|e| plotters_err("failed to build histogram chart", e))?;

    chart
        .configure_mesh()
        .x_desc(column_name)
        .y_desc("Count")
        .draw()
        .map_err(|e| plotters_err("failed to draw histogram mesh", e))?;

    chart
        .draw_series(counts.iter().enumerate().map(|(idx, count)| {
            let x0 = min + idx as f64 * width;
            let x1 = x0 + width;
            Rectangle::new([(x0, 0u32), (x1, *count)], BLUE.mix(0.55).filled())
        }))
        .map_err(|e| plotters_err("failed to draw histogram bars", e))?;

    root.present()
        .map_err(|e| plotters_err("failed to write histogram image", e))?;
    Ok(())
}
