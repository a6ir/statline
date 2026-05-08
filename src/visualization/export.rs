use std::path::{Path, PathBuf};

use polars::prelude::*;

use crate::error::Result;
use crate::profiler::ProfileReport;

use super::categorical::generate_categorical_distributions;
use super::correlation::generate_correlation_heatmap;
use super::histogram::generate_histograms;
use super::missing::generate_missing_chart;
use super::types::{VisualizationArtifacts, VisualizationOptions};
use super::utils::{ensure_dir, escape_html};

pub fn generate(
    df: Option<&DataFrame>,
    report: &ProfileReport,
    options: &VisualizationOptions,
) -> Result<VisualizationArtifacts> {
    let mut artifacts = VisualizationArtifacts::default();
    let should_chart = options.generate_charts || options.html_output.is_some();

    if should_chart {
        ensure_dir(&options.chart_dir)?;
        if let Some(frame) = df {
            artifacts
                .generated_images
                .extend(generate_histograms(frame, report, &options.chart_dir)?);
            artifacts.generated_images.extend(generate_categorical_distributions(
                frame,
                report,
                &options.chart_dir,
            )?);
        }

        if let Some(path) = generate_missing_chart(report, &options.chart_dir)? {
            artifacts.generated_images.push(path);
        }

        if let Some(corr) = &report.correlation {
            if let Some(path) = generate_correlation_heatmap(corr, &options.chart_dir)? {
                artifacts.generated_images.push(path);
            }
        }
    }

    if let Some(html_path) = &options.html_output {
        if let Some(parent) = html_path.parent() {
            ensure_dir(parent)?;
        }
        let html = build_html(report, &artifacts.generated_images, &options.chart_dir);
        std::fs::write(html_path, html)?;
        artifacts.html_output = Some(html_path.clone());
    }

    Ok(artifacts)
}

fn build_html(report: &ProfileReport, images: &[PathBuf], assets_dir: &Path) -> String {
    let mut html = String::new();
    html.push_str("<!doctype html><html><head><meta charset=\"utf-8\">");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">");
    html.push_str("<title>statline report</title>");
    html.push_str("<style>body{font-family:Arial,sans-serif;margin:24px;line-height:1.35}table{border-collapse:collapse;width:100%}th,td{border:1px solid #ddd;padding:6px 8px;font-size:13px}th{background:#f6f6f6;text-align:left}img{max-width:100%;height:auto;border:1px solid #ddd;margin:12px 0}</style>");
    html.push_str("</head><body>");

    html.push_str("<h1>Dataset Summary</h1>");
    html.push_str(&format!(
        "<p>Rows: {}<br>Columns: {}",
        report.rows, report.columns
    ));
    if let Some(sample) = report.sampled_rows {
        html.push_str(&format!("<br>Sampled rows: {sample}"));
    }
    html.push_str("</p>");

    html.push_str("<h2>Profiling Table</h2><table><thead><tr>");
    for heading in [
        "Column", "Type", "Count", "Nulls", "Null%", "Mean", "Std", "Min", "P25", "P50", "P75",
        "Max", "Unique",
    ] {
        html.push_str(&format!("<th>{heading}</th>"));
    }
    html.push_str("</tr></thead><tbody>");

    for p in &report.profiles {
        html.push_str("<tr>");
        html.push_str(&format!("<td>{}</td>", escape_html(&p.name)));
        html.push_str(&format!("<td>{}</td>", escape_html(&p.dtype)));
        html.push_str(&format!("<td>{}</td>", p.count));
        html.push_str(&format!("<td>{}</td>", p.null_count));
        html.push_str(&format!("<td>{:.2}</td>", p.null_pct));
        html.push_str(&format!("<td>{}</td>", fmt_opt_f64(p.mean)));
        html.push_str(&format!("<td>{}</td>", fmt_opt_f64(p.std)));
        html.push_str(&format!("<td>{}</td>", fmt_opt_f64(p.min)));
        html.push_str(&format!("<td>{}</td>", fmt_opt_f64(p.p25)));
        html.push_str(&format!("<td>{}</td>", fmt_opt_f64(p.p50)));
        html.push_str(&format!("<td>{}</td>", fmt_opt_f64(p.p75)));
        html.push_str(&format!("<td>{}</td>", fmt_opt_f64(p.max)));
        html.push_str(&format!("<td>{}</td>", fmt_opt_u64(p.unique_count)));
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table>");

    if !report.column_insights.is_empty() {
        html.push_str("<h2>Insights</h2><ul>");
        for insight in &report.column_insights {
            html.push_str(&format!(
                "<li><strong>{}</strong>: {}</li>",
                escape_html(&insight.column),
                escape_html(&insight.classification)
            ));
        }
        html.push_str("</ul>");
    }

    if !images.is_empty() {
        html.push_str("<h2>Charts</h2>");
        for image in images {
            let src = image
                .strip_prefix(assets_dir)
                .ok()
                .map(|p| format!("assets/{}", p.to_string_lossy()))
                .unwrap_or_else(|| image.to_string_lossy().to_string());
            html.push_str(&format!(
                "<figure><img src=\"{}\" alt=\"{}\"></figure>",
                escape_html(&src),
                escape_html(&src)
            ));
        }
    }

    html.push_str("</body></html>");
    html
}

fn fmt_opt_f64(value: Option<f64>) -> String {
    match value {
        Some(v) if v.is_finite() => format!("{v:.2}"),
        _ => "-".to_string(),
    }
}

fn fmt_opt_u64(value: Option<u64>) -> String {
    value
        .map(|v| v.to_string())
        .unwrap_or_else(|| "-".to_string())
}
