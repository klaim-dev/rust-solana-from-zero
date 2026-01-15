use crate::{
    domain::{error::DomainError, record::Record},
    pipeline::{error::BuildError, step::Step},
};
pub struct Pipeline {
    steps: Vec<Box<dyn Step>>,
}

impl Pipeline {
    pub fn new(steps: Vec<Box<dyn Step>>) -> Result<Self, BuildError> {
        if steps.is_empty() {
            return Err(BuildError::EmptyPipeline);
        }
        Ok(Self { steps })
    }
    pub fn run(&self, record: Record) -> Result<Record, DomainError> {
        let mut current = record;
        for step in &self.steps {
            current = step.apply(current)?
        }
        Ok(current)
    }

    pub fn run_all(&self, records: Vec<Record>) -> Result<Vec<Record>, DomainError> {
        let mut vec = Vec::with_capacity(records.len());
        for record in records {
            let current = self.run(record)?;
            vec.push(current);
        }
        Ok(vec)
    }
}
