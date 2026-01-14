use std::collections::HashMap;

pub trait Env {
    fn get(&self, key: &'static str) -> Option<String>;
}

pub struct OsEnv;

impl Env for OsEnv {
    fn get(&self, key: &'static str) -> Option<String> {
        std::env::var(key).ok()
    }
}

#[derive(Debug, Default)]
pub struct FakeEnv {
    vars: HashMap<String, String>,
}

impl FakeEnv {
    pub fn new(pairs: &[(&str, &str)]) -> Self {
        let mut vars = HashMap::new();
        for (k, v) in pairs {
            vars.insert(k.to_string(), v.to_string());
        }
        Self { vars }
    }
}

impl Env for FakeEnv {
    fn get(&self, key: &'static str) -> Option<String> {
        self.vars.get(key).cloned()
    }
}
