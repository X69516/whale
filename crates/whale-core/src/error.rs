
use std::fmt;

/// Common result type used throughout Whale.
pub type WhaleResult<T> = Result<T, WhaleError>;

/// Core error type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhaleError {
    InvalidState {
        component: &'static str,
        state: &'static str,
        operation: &'static str,
    },

    InvalidInput {
        field: &'static str,
        reason: &'static str,
    },

    ResourceUnavailable {
        resource: &'static str,
    },

    Internal {
        component: &'static str,
        reason: &'static str,
    },
}

impl fmt::Display for WhaleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState {
                component,
                state,
                operation,
            } => write!(
                f,
                "invalid state: component '{component}' \
                 cannot perform '{operation}' while in '{state}'"
            ),

            Self::InvalidInput { field, reason } => {
                write!(f, "invalid input '{field}': {reason}")
            }

            Self::ResourceUnavailable { resource } => {
                write!(f, "resource unavailable: {resource}")
            }

            Self::Internal { component, reason } => {
                write!(f, "internal error in '{component}': {reason}")
            }
        }
    }
}

impl std::error::Error for WhaleError {}

