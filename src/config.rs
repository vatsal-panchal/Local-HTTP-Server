/// Server configuration.
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    /// Creates a new Config with custom host and port.
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    /// Returns the full address string like "127.0.0.1:3000".
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}
