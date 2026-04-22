use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};

use crate::error::Result;
use crate::profiler::{CorrelationMatrix, ProfileReport};

const NAME_MAX_WIDTH: usize = 20;

pub fn print_table(report: &ProfileReport, color: bool, full: bool) {
    print_dataset_header(report);

    if full {
        print_full_table(report, color);
    } else {
        print_minimal_table(report, color);
    }

    if full {
        if let Some(corr) = &report.correlation {
            print_correlation_table(corr, color);
        }

        // Column Insights (clean + minimal)
        if !report.column_insights.is_empty() {
            println!("\nColumn Insights:");
            for insight in &report.column_insights {
                println!("  • {}: {}", insight.column, insight.classification);
            }
        }
    } else {
        println!("\nTip: use --full for detailed statistics");
    }
}

pub fn print_json(report: &ProfileReport) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    println!("{json}");
    Ok(())
}

fn print_dataset_header(report: &ProfileReport) {
    println!("Rows: {}", report.rows);
    println!("Columns: {}", report.columns);
    if let Some(sampled) = report.sampled_rows {
        println!("Sampled rows: {sampled}");
    }
}

fn print_minimal_table(report: &ProfileReport, color: bool) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            hcell("Column", color),
            hcell("Type", color),
            hcell("Null%", color),
            hcell("Count", color),
            hcell("Mean", color),
            hcell("Min", color),
            hcell("Max", color),
        ]);

    for p in &report.profiles {
        table.add_row(vec![
            Cell::new(truncate(&p.name, NAME_MAX_WIDTH)),
            Cell::new(truncate(&p.dtype, 12)),
            Cell::new(format!("{:.2}", p.null_pct)),
            Cell::new(p.count),
            Cell::new(fmt_opt_f64(p.mean)),
            Cell::new(fmt_opt_f64(p.min)),
            Cell::new(fmt_opt_f64(p.max)),
        ]);
    }

    println!("\nColumn Summary:");
    println!("{table}");
}

fn print_full_table(report: &ProfileReport, color: bool) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            hcell("Column", color),
            hcell("DType", color),
            hcell("Count", color),
            hcell("Nulls", color),
            hcell("Null%", color),
            hcell("Mean", color),
            hcell("Std", color),
            hcell("Min", color),
            hcell("P25", color),
            hcell("P50", color),
            hcell("P75", color),
            hcell("Max", color),
            hcell("Unique", color),
        ]);

    for p in &report.profiles {
        table.add_row(vec![
            Cell::new(truncate(&p.name, NAME_MAX_WIDTH)),
            Cell::new(truncate(&p.dtype, 18)),
            Cell::new(p.count),
            Cell::new(p.null_count),
            Cell::new(format!("{:.2}", p.null_pct)),
            Cell::new(fmt_opt_f64(p.mean)),
            Cell::new(fmt_opt_f64(p.std)),
            Cell::new(fmt_opt_f64(p.min)),
            Cell::new(fmt_opt_f64(p.p25)),
            Cell::new(fmt_opt_f64(p.p50)),
            Cell::new(fmt_opt_f64(p.p75)),
            Cell::new(fmt_opt_f64(p.max)),
            Cell::new(fmt_opt_u64(p.unique_count)),
        ]);
    }

    println!("\nColumn Summary:");
    println!("{table}");
}

fn print_correlation_table(corr: &CorrelationMatrix, color: bool) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let mut header = Vec::with_capacity(corr.columns.len() + 1);
    header.push(hcell("Column", color));
    for name in &corr.columns {
        let label = truncate(name, 12);
        header.push(hcell(&label, color));
    }
    table.set_header(header);

    for (row_idx, row_name) in corr.columns.iter().enumerate() {
        let mut row = Vec::with_capacity(corr.columns.len() + 1);
        row.push(Cell::new(truncate(row_name, 12)));
        for col_idx in 0..corr.columns.len() {
            row.push(Cell::new(fmt_opt_f64(corr.values[row_idx][col_idx])));
        }
        table.add_row(row);
    }

    println!("\nCorrelation Matrix:");
    println!("{table}");
}

fn truncate(value: &str, width: usize) -> String {
    if value.chars().count() <= width {
        value.to_string()
    } else {
        let mut s = value
            .chars()
            .take(width.saturating_sub(3))
            .collect::<String>();
        s.push_str("...");
        s
    }
}

fn fmt_opt_f64(value: Option<f64>) -> String {
    match value {
        Some(v) if v.is_finite() => format!("{v:.2}"),
        Some(_) => "nan".to_string(),
        None => "-".to_string(),
    }
}

fn fmt_opt_u64(value: Option<u64>) -> String {
    value
        .map(|v| v.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn hcell(text: &str, color: bool) -> Cell {
    let cell = Cell::new(text).add_attribute(Attribute::Bold);
    if color {
        cell.fg(Color::Cyan)
    } else {
        cell
    }
}
