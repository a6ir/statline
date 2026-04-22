use crate::error::{Result, StatlineError};
use crate::profiler::insights::generate_column_insights;
use polars::prelude::*;

pub mod corr;
pub mod insights;
pub mod profile;
pub mod types;
pub mod utils;

use corr::*;
use profile::*;
pub use types::{ColumnInsight, ColumnProfile, CorrelationMatrix, ProfileOptions, ProfileReport};

pub fn profile_dataset(lf: LazyFrame, options: ProfileOptions) -> Result<ProfileReport> {
    let schema = lf.clone().collect_schema()?;
    let working_lf = apply_sampling(lf, options.sample_rows);

    let row_count = get_row_count(&working_lf)?;
    if row_count == 0 {
        return Err(StatlineError::EmptyDataset);
    }

    let profiles = build_column_profiles(&working_lf, &schema)?;

    let correlation = if options.include_correlation {
        compute_correlation(&working_lf, &schema)?
    } else {
        None
    };

    let column_insights = if options.include_insights {
        generate_column_insights(&profiles)
    } else {
        Vec::new()
    };

    Ok(ProfileReport {
        rows: row_count,
        columns: schema.len(),
        sampled_rows: options.sample_rows,
        profiles,
        correlation,
        column_insights,
    })
}

fn apply_sampling(lf: LazyFrame, sample_rows: Option<usize>) -> LazyFrame {
    match sample_rows {
        Some(n) => lf.limit(n as IdxSize),
        None => lf,
    }
}

fn get_row_count(lf: &LazyFrame) -> Result<u64> {
    let df = lf.clone().select([len().alias("row_count")]).collect()?;
    Ok(df
        .column("row_count")?
        .get(0)?
        .try_extract::<u64>()
        .unwrap_or(0))
}
