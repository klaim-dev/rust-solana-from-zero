#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppOutput {
    Text(String),
}

impl AppOutput {
    pub fn ok() -> Self {
        Self::Text("OK".into())
    }
}
