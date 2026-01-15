use day24::pipeline::{steps_from_spec, BuildError, Pipeline, PipelineSpec};

#[test]
fn empty_pipeline_spec_returns_error() {
    let err = match PipelineSpec::parse("   ") {
        Ok(_) => panic!("expected empty pipeline error"),
        Err(err) => err,
    };
    assert!(matches!(err, BuildError::EmptyPipeline));
}

#[test]
fn unknown_step_returns_error() {
    let err = match PipelineSpec::parse("trim,wat") {
        Ok(_) => panic!("expected unknown step error"),
        Err(err) => err,
    };
    assert!(matches!(err, BuildError::UnknownStep { name } if name == "wat"));
}

#[test]
fn valid_spec_builds_pipeline() {
    let spec = PipelineSpec::parse("trim,lower_sku,normalize_space").unwrap();
    let steps = steps_from_spec(&spec).unwrap();
    let pipeline = Pipeline::new(steps).unwrap();
    let _ = pipeline;
}
