use crate::error::Result;
use polars::prelude::*;

use super::types::*;
use super::utils::*;

pub fn build_column_profiles(lf: &LazyFrame, schema: &Schema) -> Result<Vec<ColumnProfile>> {
    let df = lf.clone().collect()?;

    // Collect numeric fields once — used to build the aggregation plan.
    let numeric_fields: Vec<_> = schema
        .iter_fields()
        .filter(|f| is_numeric(f.dtype()))
        .collect();

    // Build all aggregation expressions for every numeric column in one pass.
    // The cast to Float64 lives INSIDE the lazy expression so Polars handles
    // Int8 / UInt8 / etc. internally — our Rust code never touches a raw
    // narrow-integer ChunkedArray, which was the source of the crash.
    let agg_df: Option<DataFrame> = if !numeric_fields.is_empty() {
        let exprs: Vec<Expr> = numeric_fields
            .iter()
            .flat_map(|f| {
                let n = f.name().clone();
                let c = col(n.clone()).cast(DataType::Float64);
                [
                    c.clone().mean().alias(&format!("{n}__mean")),
                    c.clone().std(1).alias(&format!("{n}__std")),
                    c.clone().min().alias(&format!("{n}__min")),
                    c.clone().max().alias(&format!("{n}__max")),
                    // Polars 0.46: quantile(f64, QuantileInterpolOptions)
                    // "quantile" feature must be enabled in Cargo.toml.
                    col(n.clone())
                        .cast(DataType::Float64)
                        .quantile(lit(0.25), QuantileMethod::Linear)
                        .alias(&format!("{n}__p25")),
                    col(n.clone())
                        .cast(DataType::Float64)
                        .quantile(lit(0.50), QuantileMethod::Linear)
                        .alias(&format!("{n}__p50")),
                    col(n.clone())
                        .cast(DataType::Float64)
                        .quantile(lit(0.75), QuantileMethod::Linear)
                        .alias(&format!("{n}__p75")),
                ]
            })
            .collect();

        df.clone().lazy().select(exprs).collect().ok() // Any plan failure → None stats; never a crash.
    } else {
        None
    };

    // Safely extract one f64 scalar from the aggregation result.
    // Returns None for null, NaN, Inf, or missing columns.
    let get_f64 = |agg: &Option<DataFrame>, suffix: &str, col_name: &str| -> Option<f64> {
        let frame = agg.as_ref()?;
        let key = format!("{col_name}__{suffix}");
        let series = frame.column(&key).ok()?;
        match series.get(0).ok()? {
            AnyValue::Float64(v) if v.is_finite() => Some(v),
            AnyValue::Float32(v) if (v as f64).is_finite() => Some(v as f64),
            _ => None,
        }
    };

    let mut profiles = Vec::with_capacity(schema.len());

    for field in schema.iter_fields() {
        let name = field.name();
        let dtype = field.dtype();
        let series = df.column(name)?;

        let count = series.len() as u64;
        let null_count = series.null_count() as u64;
        let null_pct = percent(null_count, count);

        let (mean, std, min, p25, p50, p75, max) = if is_numeric(dtype) {
            (
                get_f64(&agg_df, "mean", name),
                get_f64(&agg_df, "std", name),
                get_f64(&agg_df, "min", name),
                get_f64(&agg_df, "p25", name),
                get_f64(&agg_df, "p50", name),
                get_f64(&agg_df, "p75", name),
                get_f64(&agg_df, "max", name),
            )
        } else {
            (None, None, None, None, None, None, None)
        };

        let unique_count = if count <= UNIQUE_COUNT_THRESHOLD {
            series.n_unique().ok().map(|n| n as u64)
        } else {
            None
        };

        profiles.push(ColumnProfile {
            name: name.to_string(),
            dtype: dtype.to_string(),
            count,
            null_count,
            null_pct,
            mean,
            std,
            min,
            p25,
            p50,
            p75,
            max,
            unique_count,
            parsable_numeric_count: None,
            parsable_datetime_count: None,
        });
    }

    Ok(profiles)
}
