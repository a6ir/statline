pub fn sparkline(values: &[u64], unicode: bool) -> String {
    if values.is_empty() {
        return String::new();
    }
    let max = values.iter().copied().max().unwrap_or(1).max(1) as f64;
    if unicode {
        let glyphs = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        values
            .iter()
            .map(|v| {
                let idx = (((*v as f64 / max) * (glyphs.len() as f64 - 1.0)).round() as usize)
                    .min(glyphs.len() - 1);
                glyphs[idx]
            })
            .collect()
    } else {
        values
            .iter()
            .map(|v| if *v == 0 { '.' } else { '*' })
            .collect()
    }
}
