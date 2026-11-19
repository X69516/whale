use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TcpLimits {
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub shutdown_timeout: Duration,
}

impl Default for TcpLimits {
    fn default() -> Self {
        Self {
            max_connections: 1024,
            connection_timeout: Duration::from_secs(30),
            shutdown_timeout: Duration::from_secs(10),
        }
    }
}

impl TcpLimits {
    pub fn validate(&self) -> std::io::Result<()> {
        if self.max_connections == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "max_connections must be greater than zero",
            ));
        }

        if self.connection_timeout.is_zero() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "connection_timeout must be greater than zero",
            ));
        }

        Ok(())
    }
}
