use std::path::Path;

use plotters::prelude::*;

use crate::error::{Result, StatlineError};

pub const CHART_SIZE: (u32, u32) = (960, 640);
pub const TOP_K_CATEGORIES: usize = 10;

pub fn ensure_dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub fn sanitize_filename(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            out.push(ch);
        } else if ch.is_whitespace() || ch == '.' {
            out.push('_');
        }
    }
    if out.is_empty() {
        "column".to_string()
    } else {
        out
    }
}

pub fn plotters_err<E: std::fmt::Debug>(context: &str, err: E) -> StatlineError {
    StatlineError::Message(format!("{context}: {err:?}"))
}

pub fn clamp_f64(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

pub fn heat_color(value: f64) -> RGBColor {
    let v = clamp_f64(value, -1.0, 1.0);
    if v >= 0.0 {
        let strength = (v * 255.0).round() as u8;
        RGBColor(255, 255u8.saturating_sub(strength), 255u8.saturating_sub(strength))
    } else {
        let strength = ((-v) * 255.0).round() as u8;
        RGBColor(255u8.saturating_sub(strength), 255u8.saturating_sub(strength), 255)
    }
}

pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
