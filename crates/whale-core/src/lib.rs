
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

pub mod error;
pub mod id;
pub mod lifecycle;
pub mod version;

pub use error::{WhaleError, WhaleResult};
pub use id::ComponentId;
pub use lifecycle::LifecycleState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_version_is_available() {
        assert!(!version::WHALE_VERSION.is_empty());
    }

    #[test]
    fn protocol_version_is_valid() {
        assert!(version::PROTOCOL_VERSION > 0);
    }
}

