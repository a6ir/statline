use super::types::*;
use crate::error::Result;
use crate::profiler::utils::is_numeric;
use polars::prelude::*;

pub fn compute_correlation(lf: &LazyFrame, schema: &Schema) -> Result<Option<CorrelationMatrix>> {
    let numeric_cols: Vec<String> = schema
        .iter_fields()
        .filter(|f| is_numeric(f.dtype()))
        .map(|f| f.name().to_string())
        .collect();

    let n = numeric_cols.len();
    if n < 2 {
        return Ok(None);
    }

    // Build pairwise correlation expressions.
    // We compute only the upper triangle (including diagonal) to save time.
    let mut exprs = Vec::with_capacity(n * (n + 1) / 2);
    for i in 0..n {
        for j in i..n {
            let col_i = col(&numeric_cols[i]).cast(DataType::Float64);
            let col_j = col(&numeric_cols[j]).cast(DataType::Float64);
            exprs.push(pearson_corr(col_i, col_j).alias(&format!("c_{i}_{j}")));
        }
    }

    // Collect all correlations in one pass.
    let res = lf.clone().select(exprs).collect()?;

    let mut values = vec![vec![None; n]; n];
    for i in 0..n {
        for j in i..n {
            let alias = format!("c_{i}_{j}");
            let series = res.column(&alias)?;
            let val = match series.get(0)? {
                AnyValue::Float64(v) if v.is_finite() => Some(v),
                AnyValue::Float32(v) if (v as f64).is_finite() => Some(v as f64),
                _ => None,
            };
            values[i][j] = val;
            values[j][i] = val;
        }
    }

    Ok(Some(CorrelationMatrix {
        columns: numeric_cols,
        values,
    }))
}
