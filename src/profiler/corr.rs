use super::types::*;
use crate::error::Result;
use polars::prelude::*;

pub fn compute_correlation(_lf: &LazyFrame, _schema: &Schema) -> Result<Option<CorrelationMatrix>> {
    // MVP: disabled
    Ok(None)
}
