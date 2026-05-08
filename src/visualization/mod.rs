use std::path::Path;

use polars::prelude::*;

use crate::error::Result;
use crate::profiler::ProfileReport;

pub mod categorical;
pub mod correlation;
pub mod export;
pub mod histogram;
pub mod missing;
pub mod terminal;
pub mod types;
pub mod utils;

pub use types::{VisualizationArtifacts, VisualizationOptions};

pub fn generate_visualizations(
    df: Option<&DataFrame>,
    report: &ProfileReport,
    options: &VisualizationOptions,
) -> Result<VisualizationArtifacts> {
    export::generate(df, report, options)
}

pub fn default_assets_dir(html_path: &Path) -> std::path::PathBuf {
    html_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("assets")
}
