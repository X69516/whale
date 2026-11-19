
use std::fmt;
use std::num::NonZeroU64;

/// Unique identifier of a runtime component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(NonZeroU64);

impl ComponentId {
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl TryFrom<u64> for ComponentId {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value = NonZeroU64::new(value)
            .ok_or("component ID cannot be zero")?;

        Ok(Self(value))
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_component_id() {
        let id = ComponentId::try_from(42).unwrap();

        assert_eq!(id.get(), 42);
    }

    #[test]
    fn rejects_zero_id() {
        assert!(ComponentId::try_from(0).is_err());
    }

    #[test]
    fn component_id_is_copy() {
        let id = ComponentId::try_from(1).unwrap();
        let copied = id;

        assert_eq!(id, copied);
    }
}

