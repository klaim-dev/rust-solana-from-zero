#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("empty pipeline")]
    EmptyPipeline,
    #[error("unknown step: {name}")]
    UnknownStep { name: String },
    #[error("empty step token")]
    EmptyStepToken,
}
