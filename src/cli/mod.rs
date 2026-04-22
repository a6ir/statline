use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(name = "statline")]
#[command(author, version, about = "Fast CLI EDA for CSV and Parquet")]
pub struct CliArgs {
    /// Input dataset path (.csv or .parquet)
    pub input: PathBuf,

    /// Compute numeric correlation matrix
    #[arg(long)]
    pub corr: bool,

    /// Emit JSON output
    #[arg(long)]
    pub json: bool,

    /// Limit profiling to first N rows
    #[arg(long, value_name = "N", value_parser = clap::value_parser!(usize))]
    pub sample: Option<usize>,

    /// Disable ANSI colors in table output
    #[arg(long)]
    pub no_color: bool,

    /// Print heuristic insights based on computed statistics
    #[arg(long)]
    pub no_insights: bool,

    /// Show full statistics (std, percentiles, unique, correlation, insights)
    #[arg(long)]
    pub full: bool,
}
