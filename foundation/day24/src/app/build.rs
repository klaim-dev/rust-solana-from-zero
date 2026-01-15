use crate::app::{error::AppError, state::AppState};
use crate::config::models::Config;
use crate::pipeline::{steps_from_spec, Pipeline, PipelineSpec};

pub fn build_app(cfg: Config) -> Result<AppState, AppError> {
    let spec = PipelineSpec::parse(&cfg.pipeline_raw)?;
    let steps = steps_from_spec(&spec)?;
    let pipeline = Pipeline::new(steps)?;
    Ok(AppState { pipeline })
}
