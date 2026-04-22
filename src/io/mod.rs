use std::path::{Path, PathBuf};

use polars::prelude::*;

use crate::error::{Result, StatlineError};

#[derive(Debug, Clone, Copy)]
enum InputFormat {
    Csv,
    Parquet,
}

pub fn scan_dataset(path: &Path) -> Result<LazyFrame> {
    validate_input_path(path)?;

    let format = detect_format(path)?;

    let lf = match format {
        InputFormat::Csv => LazyCsvReader::new(path)
            .with_ignore_errors(true)
            .with_infer_schema_length(Some(10000))
            .finish()?,

        InputFormat::Parquet => LazyFrame::scan_parquet(path, ScanArgsParquet::default())?,
    };

    Ok(lf)
}

fn validate_input_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(StatlineError::InputNotFound(path.to_path_buf()));
    }

    if !path.is_file() {
        return Err(StatlineError::InputNotAFile(path.to_path_buf()));
    }

    Ok(())
}

fn detect_format(path: &Path) -> Result<InputFormat> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase());

    match ext.as_deref() {
        Some("csv") => Ok(InputFormat::Csv),
        Some("parquet") | Some("pq") => Ok(InputFormat::Parquet),
        _ => Err(StatlineError::UnsupportedFormat(PathBuf::from(path))),
    }
}
