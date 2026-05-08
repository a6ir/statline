use polars::prelude::DataFrame;

use crate::profiler::ProfileReport;

pub mod bars;
pub mod categorical;
pub mod histogram;
pub mod sparkline;
pub mod utils;

pub fn render_terminal_charts(report: &ProfileReport, df: Option<&DataFrame>) -> Option<String> {
    let width = utils::terminal_width();
    let charset = if utils::supports_unicode() {
        bars::BarCharset::Unicode
    } else {
        bars::BarCharset::Ascii
    };

    let mut sections = Vec::new();

    if let Some(frame) = df {
        let hist = histogram::render_numeric_histograms(frame, report, width, charset);
        if !hist.is_empty() {
            sections.push(hist);
        }

        let cat = categorical::render_categorical_distributions(frame, report, width, charset);
        if !cat.is_empty() {
            sections.push(cat);
        }
    }

    let missing = bars::render_missing_bars(report, width, charset);
    if !missing.is_empty() {
        sections.push(missing);
    }

    let corr = bars::render_correlation_strength(report, width, charset);
    if !corr.is_empty() {
        sections.push(corr);
    }

    if sections.is_empty() {
        None
    } else {
        Some(format!("\nTerminal Charts:\n{}", sections.join("\n\n")))
    }
}
