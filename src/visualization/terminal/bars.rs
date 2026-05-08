use crate::profiler::ProfileReport;

use super::utils::truncate;

#[derive(Clone, Copy)]
pub enum BarCharset {
    Unicode,
    Ascii,
}

pub fn bar(value: f64, max: f64, width: usize, charset: BarCharset) -> String {
    let width = width.max(1);
    let fill = if max <= 0.0 {
        0
    } else {
        ((value.max(0.0) / max) * width as f64).round() as usize
    }
    .min(width);
    let ch = match charset {
        BarCharset::Unicode => '█',
        BarCharset::Ascii => '#',
    };
    std::iter::repeat_n(ch, fill).collect()
}

pub fn render_missing_bars(report: &ProfileReport, width: usize, charset: BarCharset) -> String {
    let mut rows: Vec<_> = report
        .profiles
        .iter()
        .map(|p| (p.name.as_str(), p.null_pct))
        .collect();
    rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    rows.truncate(12);

    if rows.is_empty() {
        return String::new();
    }

    let label_w = 16usize.min(width / 4).max(8);
    let bar_w = (width.saturating_sub(label_w + 10)).clamp(10, 40);
    let mut out = String::from("Missing Values");
    for (name, pct) in rows {
        let b = bar(pct, 100.0, bar_w, charset);
        out.push_str(&format!(
            "\n{:<label_w$} {} {:>5.1}%",
            truncate(name, label_w),
            b,
            pct,
            label_w = label_w
        ));
    }
    out
}

pub fn render_correlation_strength(
    report: &ProfileReport,
    width: usize,
    charset: BarCharset,
) -> String {
    let Some(corr) = &report.correlation else {
        return String::new();
    };
    let n = corr.columns.len();
    if n < 2 {
        return String::new();
    }

    let mut pairs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            if let Some(v) = corr.values[i][j] {
                if v.is_finite() {
                    pairs.push((i, j, v.abs(), v));
                }
            }
        }
    }
    pairs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    pairs.truncate(10);
    if pairs.is_empty() {
        return String::new();
    }

    let label_w = 30usize.min(width / 2).max(14);
    let bar_w = (width.saturating_sub(label_w + 10)).clamp(8, 30);
    let mut out = String::from("Correlation Strength");
    for (i, j, abs_v, raw_v) in pairs {
        let label = format!("{} <-> {}", corr.columns[i], corr.columns[j]);
        let b = bar(abs_v, 1.0, bar_w, charset);
        out.push_str(&format!(
            "\n{:<label_w$} {} {:>5.2}",
            truncate(&label, label_w),
            b,
            raw_v,
            label_w = label_w
        ));
    }
    out
}
