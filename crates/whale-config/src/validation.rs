use crate::error::ConfigError;

pub fn validate_non_empty(field: &'static str, value: &str) -> Result<(), ConfigError> {
    if value.trim().is_empty() {
        return Err(ConfigError::InvalidValue {
            field,
            reason: "value cannot be empty",
        });
    }

    Ok(())
}

pub fn validate_range(
    field: &'static str,
    value: u64,
    min: u64,
    max: u64,
) -> Result<(), ConfigError> {
    if value < min || value > max {
        return Err(ConfigError::OutOfRange { field, min, max });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_non_empty_values() {
        assert!(validate_non_empty("name", "whale").is_ok());
    }

    #[test]
    fn rejects_empty_values() {
        assert!(validate_non_empty("name", " ").is_err());
    }

    #[test]
    fn accepts_values_inside_range() {
        assert!(validate_range("workers", 10, 1, 100).is_ok());
    }

    #[test]
    fn rejects_values_outside_range() {
        assert!(validate_range("workers", 0, 1, 100).is_err());
    }
}
