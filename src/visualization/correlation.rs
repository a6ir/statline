use std::path::{Path, PathBuf};

use plotters::prelude::*;

use crate::error::Result;
use crate::profiler::CorrelationMatrix;

use super::utils::{heat_color, plotters_err, CHART_SIZE};

pub fn generate_correlation_heatmap(corr: &CorrelationMatrix, chart_dir: &Path) -> Result<Option<PathBuf>> {
    let n = corr.columns.len();
    if n == 0 {
        return Ok(None);
    }

    let path = chart_dir.join("correlation_heatmap.png");
    render_heatmap(&path, corr)?;
    Ok(Some(path))
}

fn render_heatmap(path: &Path, corr: &CorrelationMatrix) -> Result<()> {
    let n = corr.columns.len();
    let root = BitMapBackend::new(path, CHART_SIZE).into_drawing_area();
    root.fill(&WHITE)
        .map_err(|e| plotters_err("failed to fill correlation background", e))?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Correlation Heatmap", ("sans-serif", 28))
        .margin(20)
        .x_label_area_size(90)
        .y_label_area_size(110)
        .build_cartesian_2d(0i32..n as i32, 0i32..n as i32)
        .map_err(|e| plotters_err("failed to build correlation chart", e))?;

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("Columns")
        .y_desc("Columns")
        .x_labels(n)
        .y_labels(n)
        .x_label_formatter(&|idx| {
            corr.columns
                .get(*idx as usize)
                .map(|s| truncate(s, 10))
                .unwrap_or_default()
        })
        .y_label_formatter(&|idx| {
            corr.columns
                .get(*idx as usize)
                .map(|s| truncate(s, 10))
                .unwrap_or_default()
        })
        .draw()
        .map_err(|e| plotters_err("failed to draw correlation mesh", e))?;

    for row in 0..n {
        for col in 0..n {
            let val = corr
                .values
                .get(row)
                .and_then(|r| r.get(col))
                .copied()
                .flatten();
            let color = match val {
                Some(v) if v.is_finite() => heat_color(v),
                _ => RGBColor(220, 220, 220),
            };

            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(col as i32, row as i32), (col as i32 + 1, row as i32 + 1)],
                    color.filled(),
                )))
                .map_err(|e| plotters_err("failed to draw correlation cell", e))?;

            let label = match val {
                Some(v) if v.is_finite() => format!("{v:.2}"),
                _ => "nan".to_string(),
            };
            chart
                .draw_series(std::iter::once(Text::new(
                    label,
                    (col as i32, row as i32),
                    ("sans-serif", 14).into_font().color(&BLACK),
                )))
                .map_err(|e| plotters_err("failed to draw correlation label", e))?;
        }
    }

    root.present()
        .map_err(|e| plotters_err("failed to write correlation image", e))?;
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
