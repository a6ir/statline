use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct VisualizationOptions {
    pub generate_charts: bool,
    pub chart_dir: PathBuf,
    pub html_output: Option<PathBuf>,
}

#[derive(Debug, Clone, Default)]
pub struct VisualizationArtifacts {
    pub generated_images: Vec<PathBuf>,
    pub html_output: Option<PathBuf>,
}
