use day24::domain::record::{Name, PriceCents, Record, Sku};
use day24::pipeline::{steps_from_spec, Pipeline, PipelineSpec};

fn build_pipeline(spec: &str) -> Pipeline {
    let spec = PipelineSpec::parse(spec).unwrap();
    let steps = steps_from_spec(&spec).unwrap();
    Pipeline::new(steps).unwrap()
}

fn make_record(sku: &str, name: &str, price: u64) -> Record {
    Record::new(
        Sku::new(sku.to_string()).unwrap(),
        Name::new(name.to_string()).unwrap(),
        PriceCents::new(price).unwrap(),
    )
}

fn name_from(record: Record) -> String {
    let (_sku, name, _price) = record.into_parts();
    name.get().to_string()
}

fn sku_from(record: Record) -> String {
    let (sku, _name, _price) = record.into_parts();
    sku.get().to_string()
}

#[test]
fn trim_name_happy() {
    let pipeline = build_pipeline("trim");
    let record = make_record("SKU-1", "  My Name  ", 100);
    let output = pipeline.run(record).unwrap();
    assert_eq!(name_from(output), "My Name");
}

#[test]
fn normalize_space_name_happy() {
    let pipeline = build_pipeline("normalize_space");
    let record = make_record("SKU-1", "Many   spaces  here", 100);
    let output = pipeline.run(record).unwrap();
    assert_eq!(name_from(output), "Many spaces here");
}

#[test]
fn lower_sku_happy() {
    let pipeline = build_pipeline("lower_sku");
    let record = make_record("SKU-XYZ", "Name", 100);
    let output = pipeline.run(record).unwrap();
    assert_eq!(sku_from(output), "sku-xyz");
}

#[test]
fn space_to_underscore_name_happy() {
    let pipeline = build_pipeline("space_to_underscore");
    let record = make_record("SKU-1", "A  B", 100);
    let output = pipeline.run(record).unwrap();
    assert_eq!(name_from(output), "A__B");
}

#[test]
fn spec_sequence_changes_output() {
    let pipeline_first = build_pipeline("normalize_space,space_to_underscore");
    let pipeline_second = build_pipeline("space_to_underscore,normalize_space");

    let record_first = make_record("SKU-1", "  A   B  ", 100);
    let record_second = make_record("SKU-1", "  A   B  ", 100);

    let name_first = name_from(pipeline_first.run(record_first).unwrap());
    let name_second = name_from(pipeline_second.run(record_second).unwrap());

    assert_ne!(name_first, name_second);
}
