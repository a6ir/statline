use polars::prelude::*;

// Previously typed as u64 in some call sites and cast with `as u64` at the
// comparison site, which worked but was inconsistent. Declared as u64 here
// so comparisons like `count <= UNIQUE_COUNT_THRESHOLD` compile without casts.
pub const UNIQUE_COUNT_THRESHOLD: u64 = 5_000_000;

pub fn percent(part: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

pub fn is_numeric(dtype: &DataType) -> bool {
    matches!(
        dtype,
        DataType::Int8
            | DataType::Int16
            | DataType::Int32
            | DataType::Int64
            | DataType::UInt8
            | DataType::UInt16
            | DataType::UInt32
            | DataType::UInt64
            | DataType::Float32
            | DataType::Float64
    )
}

pub fn is_semantic_id(name: &str) -> bool {
    let n = name.to_lowercase();
    n == "id" || n.ends_with("_id") || n.contains("uuid")
}
