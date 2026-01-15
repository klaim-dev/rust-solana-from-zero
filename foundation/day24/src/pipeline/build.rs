use crate::pipeline::{
    error::BuildError,
    spec::{PipelineSpec, StepKind},
    step::{LowerSku, NormalizeSpaceName, SpaceToUnderscoreName, Step, TrimName},
};

pub fn steps_from_spec(spec: &PipelineSpec) -> Result<Vec<Box<dyn Step>>, BuildError> {
    let mut steps = Vec::with_capacity(spec.steps.len());
    for kind in &spec.steps {
        let step = build_step(kind);
        steps.push(step);
    }

    if steps.is_empty() {
        return Err(BuildError::EmptyPipeline);
    }
    Ok(steps)
}

fn build_step(kind: &StepKind) -> Box<dyn Step> {
    match kind {
        StepKind::LowerSku => Box::new(LowerSku),
        StepKind::NormalizeSpaceName => Box::new(NormalizeSpaceName),
        StepKind::SpaceToUnderscoreName => Box::new(SpaceToUnderscoreName),
        StepKind::TrimName => Box::new(TrimName),
    }
}
