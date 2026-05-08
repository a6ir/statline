use std::collections::HashMap;

use polars::prelude::{DataFrame, DataType};

use crate::profiler::ProfileReport;
use crate::profiler::utils::is_numeric;

use super::bars::{bar, BarCharset};
use super::utils::truncate;

pub fn render_categorical_distributions(
    df: &DataFrame,
    report: &ProfileReport,
    width: usize,
    charset: BarCharset,
) -> String {
    let mut sections = Vec::new();

    for profile in report.profiles.iter().take(6) {
        let Ok(col) = df.column(&profile.name) else {
            continue;
        };
        if is_numeric(col.dtype()) {
            continue;
        }
        let Ok(casted) = col.cast(&DataType::String) else {
            continue;
        };
        let Ok(utf8) = casted.str() else {
            continue;
        };

        let mut counts: HashMap<String, u64> = HashMap::new();
        for v in utf8.into_iter().flatten() {
            *counts.entry(v.to_string()).or_insert(0) += 1;
        }
        if counts.is_empty() {
            continue;
        }

        let mut items: Vec<(String, u64)> = counts.into_iter().collect();
        items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        items.truncate(6);
        sections.push(render_one(&profile.name, &items, width, charset));
    }

    sections.join("\n\n")
}

fn render_one(name: &str, items: &[(String, u64)], width: usize, charset: BarCharset) -> String {
    let total: u64 = items.iter().map(|(_, c)| *c).sum();
    let max_count = items.iter().map(|(_, c)| *c).max().unwrap_or(1).max(1) as f64;

    let label_w = 16usize.min(width / 3).max(10);
    let bar_w = (width.saturating_sub(label_w + 11)).clamp(8, 34);

    let mut out = truncate(name, 36);
    for (label, count) in items {
        let pct = if total == 0 {
            0.0
        } else {
            (*count as f64 * 100.0) / total as f64
        };
        let b = bar(*count as f64, max_count, bar_w, charset);
        out.push_str(&format!(
            "\n{:<label_w$} {} {:>5.1}%",
            truncate(label, label_w),
            b,
            pct,
            label_w = label_w
        ));
    }
    out
}
