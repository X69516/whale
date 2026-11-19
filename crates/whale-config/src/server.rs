use std::time::Duration;

use crate::{
    error::ConfigError,
    validation::{validate_non_empty, validate_range},
};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub listen_address: String,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub shutdown_timeout: Duration,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            listen_address: "127.0.0.1:8080".to_owned(),
            max_connections: 1024,
            connection_timeout: Duration::from_secs(30),
            shutdown_timeout: Duration::from_secs(10),
        }
    }
}

impl ServerConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        validate_non_empty("listen_address", &self.listen_address)?;

        self.listen_address
            .parse::<std::net::SocketAddr>()
            .map_err(|_| ConfigError::InvalidValue {
                field: "listen_address",
                reason: "expected a valid socket address",
            })?;

        validate_range("max_connections", self.max_connections as u64, 1, 65_535)?;

        validate_range(
            "connection_timeout",
            self.connection_timeout.as_secs(),
            1,
            3600,
        )?;

        validate_range("shutdown_timeout", self.shutdown_timeout.as_secs(), 1, 300)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        assert!(ServerConfig::default().validate().is_ok());
    }

    #[test]
    fn rejects_invalid_address() {
        let config = ServerConfig {
            listen_address: "invalid-address".to_owned(),
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn rejects_zero_connections() {
        let config = ServerConfig {
            max_connections: 0,
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }
}
