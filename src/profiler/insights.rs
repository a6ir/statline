use crate::profiler::types::{ColumnInsight, ColumnProfile};

pub fn generate_column_insights(profiles: &[ColumnProfile]) -> Vec<ColumnInsight> {
    profiles
        .iter()
        .map(|p| ColumnInsight {
            column: p.name.clone(),
            classification: classify(p),
        })
        .collect()
}

fn classify(p: &ColumnProfile) -> String {
    match p.dtype.as_str() {
        "i64" | "i32" | "f64" | "f32" => {
            if is_identifier(p) {
                "Identifier".to_string()
            } else {
                "Numeric".to_string()
            }
        }

        "str" => {
            let unique = p.unique_count.unwrap_or(0);

            // ✅ NEW: boolean-like detection
            if unique == 2 {
                "Boolean/Categorical".to_string()
            } else if unique < 20 {
                "Categorical".to_string()
            } else {
                "Text".to_string()
            }
        }

        _ => "Unknown".to_string(),
    }
}

fn is_identifier(p: &ColumnProfile) -> bool {
    p.unique_count.unwrap_or(0) == p.count as u64
}
