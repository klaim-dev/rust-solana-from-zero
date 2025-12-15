#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Dev,
    Prod,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    port: u16,
    debug: bool,
    db_url: String,
    max_connections: u32,
    mode: Mode,
}

impl Config {
    pub(crate) fn new(
        port: u16,
        debug: bool,
        db_url: String,
        max_connections: u32,
        mode: Mode,
    ) -> Self {
        Self {
            port,
            debug,
            db_url,
            max_connections,
            mode,
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }
}
