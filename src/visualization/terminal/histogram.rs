use polars::prelude::{DataFrame, DataType};

use crate::profiler::ProfileReport;
use crate::profiler::utils::is_numeric;

use super::bars::{bar, BarCharset};
use super::sparkline::sparkline;
use super::utils::{numeric_column_names, truncate};

pub fn render_numeric_histograms(
    df: &DataFrame,
    report: &ProfileReport,
    width: usize,
    charset: BarCharset,
) -> String {
    let mut sections = Vec::new();
    for name in numeric_column_names(&report.profiles).into_iter().take(4) {
        let Ok(col) = df.column(name) else {
            continue;
        };
        if !is_numeric(col.dtype()) {
            continue;
        }
        let Ok(casted) = col.cast(&DataType::Float64) else {
            continue;
        };
        let Ok(ca) = casted.f64() else {
            continue;
        };
        let vals: Vec<f64> = ca.into_iter().flatten().filter(|v| v.is_finite()).collect();
        if vals.is_empty() {
            continue;
        }
        let rendered = render_one(name, &vals, width, charset);
        if !rendered.is_empty() {
            sections.push(rendered);
        }
    }
    sections.join("\n\n")
}

fn render_one(name: &str, values: &[f64], width: usize, charset: BarCharset) -> String {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !min.is_finite() || !max.is_finite() {
        return String::new();
    }

    let bins = 8usize.min(values.len().max(1));
    let w = if (max - min).abs() < f64::EPSILON {
        1.0
    } else {
        (max - min) / bins as f64
    };
    let mut counts = vec![0u64; bins];
    for v in values {
        let mut idx = if w > 0.0 {
            ((v - min) / w).floor() as isize
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

    let max_count = counts.iter().copied().max().unwrap_or(1).max(1) as f64;
    let label_w = 16usize.min(width / 4).max(10);
    let bar_w = (width.saturating_sub(label_w + 10)).clamp(10, 40);
    let mut out = format!("Numeric Distribution: {}", truncate(name, 32));
    out.push_str(&format!(
        "\ntrend: {}",
        sparkline(&counts, matches!(charset, BarCharset::Unicode))
    ));

    for (idx, count) in counts.iter().enumerate() {
        let lo = min + idx as f64 * w;
        let hi = lo + w;
        let label = format!("{:.2}-{:.2}", lo, hi);
        let b = bar(*count as f64, max_count, bar_w, charset);
        out.push_str(&format!(
            "\n{:<label_w$} {}",
            truncate(&label, label_w),
            b,
            label_w = label_w
        ));
    }
    out
}
