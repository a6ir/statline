use std::path::PathBuf;

use clap::{ArgAction, Parser};

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

    /// Generate chart PNG assets
    #[arg(long)]
    pub charts: bool,

    /// Enable inline terminal charts (text-based)
    #[arg(long = "terminal-charts", default_value_t = true, action = ArgAction::SetTrue)]
    pub terminal_charts: bool,

    /// Disable inline terminal charts
    #[arg(long = "no-terminal-charts", action = ArgAction::SetTrue)]
    pub no_terminal_charts: bool,

    /// Output directory for generated chart assets
    #[arg(long, value_name = "DIR", default_value = "report/assets")]
    pub chart_dir: PathBuf,

    /// Generate standalone HTML report file
    #[arg(long, value_name = "FILE")]
    pub html: Option<PathBuf>,
}
