use std::process::ExitCode;

use clap::Parser;

use statline::cli::CliArgs;
use statline::error::{Result, StatlineError};
use statline::io;
use statline::output;
use statline::profiler::{self, ProfileOptions};

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
        input_lf,
        ProfileOptions {
            include_correlation: args.corr,
            sample_rows: args.sample,
            include_insights: !args.no_insights,
        },
    )?;

    // -----------------------------
    // Output
    // -----------------------------

    if args.json {
        output::print_json(&report)?;
    } else {
        output::print_table(&report, !args.no_color, args.full);
    }

    Ok(())
}
