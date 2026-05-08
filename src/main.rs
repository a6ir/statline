use std::process::ExitCode;

use clap::Parser;

use statline::cli::CliArgs;
use statline::error::{Result, StatlineError};
use statline::io;
use statline::output;
use statline::profiler::{self, ProfileOptions};
use statline::visualization::{self, VisualizationOptions};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let args = CliArgs::parse();

    // -----------------------------
    // Input validation
    // -----------------------------

    if let Some(sample) = args.sample {
        if sample == 0 {
            return Err(StatlineError::Message(
                "--sample must be greater than 0".to_string(),
            ));
        }
    }

    if args.json && args.no_color {
        // no_color is irrelevant in JSON mode → prevent confusion
        return Err(StatlineError::Message(
            "--no-color cannot be used with --json".to_string(),
        ));
    }

    // -----------------------------
    // Load dataset
    // -----------------------------

    let input_lf = io::scan_dataset(&args.input)?;

    // -----------------------------
    // Profile dataset
    // -----------------------------

    let report = profiler::profile_dataset(
        input_lf.clone(),
        ProfileOptions {
            include_correlation: args.corr,
            sample_rows: args.sample,
            include_insights: !args.no_insights,
        },
    )?;

    // -----------------------------
    // Optional visualization generation
    // -----------------------------

    let default_chart_dir = std::path::PathBuf::from("report/assets");
    let chart_dir = if let Some(html_path) = &args.html {
        if args.chart_dir == default_chart_dir {
            visualization::default_assets_dir(html_path)
        } else {
            args.chart_dir.clone()
        }
    } else {
        args.chart_dir.clone()
    };

    let terminal_charts_enabled = args.terminal_charts && !args.no_terminal_charts;
    let needs_chart_df = (args.charts || args.html.is_some())
        || (!args.json && args.full && terminal_charts_enabled);
    let mut chart_df: Option<polars::prelude::DataFrame> = None;

    if needs_chart_df {
        let sampled_lf = match args.sample {
            Some(n) => input_lf.limit(n as polars::prelude::IdxSize),
            None => input_lf.clone(),
        };
        match sampled_lf.collect() {
            Ok(collected_df) => {
                chart_df = Some(collected_df);
            }
            Err(err) => {
                eprintln!("warning: chart data collection failed: {err}");
            }
        }
    }

    if args.charts || args.html.is_some() {
        if let Some(collected_df) = chart_df.as_ref() {
            let viz_options = VisualizationOptions {
                generate_charts: args.charts,
                chart_dir: chart_dir.clone(),
                html_output: args.html.clone(),
            };

            if let Err(err) =
                visualization::generate_visualizations(Some(collected_df), &report, &viz_options)
            {
                eprintln!("warning: visualization generation failed: {err}");
            }
        } else {
            let viz_options = VisualizationOptions {
                generate_charts: args.charts,
                chart_dir: chart_dir.clone(),
                html_output: args.html.clone(),
            };

            if let Err(err) = visualization::generate_visualizations(None, &report, &viz_options) {
                eprintln!("warning: visualization generation failed: {err}");
            }
        }
    }

    // -----------------------------
    // Output
    // -----------------------------

    if args.json {
        output::print_json(&report)?;
    } else {
        output::print_table(
            &report,
            chart_df.as_ref(),
            !args.no_color,
            args.full,
            terminal_charts_enabled,
        );
    }

    Ok(())
}
