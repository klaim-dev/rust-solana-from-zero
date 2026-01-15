use crate::pipeline::error::BuildError;
pub enum StepKind {
    TrimName,
    LowerSku,
    NormalizeSpaceName,
    SpaceToUnderscoreName,
}

pub struct PipelineSpec {
    pub steps: Vec<StepKind>,
}

impl PipelineSpec {
    pub fn parse(spec_str: &str) -> Result<Self, BuildError> {
        if spec_str.trim().is_empty() {
            return Err(BuildError::EmptyPipeline);
        }

        let steps = spec_str
            .split(',')
            .map(|step| match step.trim() {
                "" => Err(BuildError::EmptyStepToken),
                "trim" => Ok(StepKind::TrimName),
                "lower_sku" => Ok(StepKind::LowerSku),
                "normalize_space" => Ok(StepKind::NormalizeSpaceName),
                "space_to_underscore" => Ok(StepKind::SpaceToUnderscoreName),
                other => Err(BuildError::UnknownStep {
                    name: other.to_string(),
                }),
            })
            .collect::<Result<Vec<_>, _>>()?;

        if steps.is_empty() {
            return Err(BuildError::EmptyPipeline);
        }
        Ok(Self { steps })
    }
}
