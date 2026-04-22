use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub struct ProfileOptions {
    pub include_correlation: bool,
    pub sample_rows: Option<usize>,
    pub include_insights: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ColumnProfile {
    pub name: String,
    pub dtype: String,
    pub count: u64,
    pub null_count: u64,
    pub null_pct: f64,
    pub mean: Option<f64>,
    pub std: Option<f64>,
    pub min: Option<f64>,
    pub p25: Option<f64>,
    pub p50: Option<f64>,
    pub p75: Option<f64>,
    pub max: Option<f64>,
    pub unique_count: Option<u64>,
    pub parsable_numeric_count: Option<u64>,
    pub parsable_datetime_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CorrelationMatrix {
    pub columns: Vec<String>,
    pub values: Vec<Vec<Option<f64>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ColumnInsight {
    pub column: String,
    pub classification: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileReport {
    pub rows: u64,
    pub columns: usize,
    pub sampled_rows: Option<usize>,
    pub profiles: Vec<ColumnProfile>,
    pub correlation: Option<CorrelationMatrix>,
    pub column_insights: Vec<ColumnInsight>,
}
