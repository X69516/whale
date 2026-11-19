#![forbid(unsafe_code)]

use std::fmt;

/// Errors produced while loading or validating Whale configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    MissingValue {
        field: &'static str,
    },

    InvalidValue {
        field: &'static str,
        reason: &'static str,
    },

    OutOfRange {
        field: &'static str,
        min: u64,
        max: u64,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingValue { field } => {
                write!(f, "missing configuration value: {field}")
            }

            Self::InvalidValue { field, reason } => {
                write!(f, "invalid configuration value '{field}': {reason}")
            }

            Self::OutOfRange { field, min, max } => {
                write!(
                    f,
                    "configuration value '{field}' must be between {min} and {max}"
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}
