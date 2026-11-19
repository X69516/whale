use crate::{error::ConfigError, validation::validate_range};

#[derive(Debug, Clone)]
pub struct SupervisorConfig {
    pub max_workers: usize,
    pub worker_shutdown_timeout_secs: u64,
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        Self {
            max_workers: 64,
            worker_shutdown_timeout_secs: 10,
        }
    }
}

impl SupervisorConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        validate_range("max_workers", self.max_workers as u64, 1, 4096)?;

        validate_range(
            "worker_shutdown_timeout_secs",
            self.worker_shutdown_timeout_secs,
            1,
            300,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        assert!(SupervisorConfig::default().validate().is_ok());
    }

    #[test]
    fn rejects_zero_workers() {
        let config = SupervisorConfig {
            max_workers: 0,
            ..Default::default()
        };

        assert!(config.validate().is_err());
    }
}
