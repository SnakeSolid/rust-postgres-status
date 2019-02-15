use super::ConfigError;
use super::ConfigRef;
use super::ConfigResult;

#[allow(clippy::needless_pass_by_value)]
pub fn validate(config: ConfigRef) -> ConfigResult<()> {
    validate_number(config.update_interval(), "update_interval")?;
    validate_number(config.server().disk().capacity(), "capacity")?;
    validate_number(config.server().disk().soft_threshold(), "soft_threshold")?;
    validate_number(config.server().disk().hard_threshold(), "hard_threshold")?;

    Ok(())
}

fn validate_number(value: u64, name: &str) -> ConfigResult<()> {
    if value > 0 {
        Ok(())
    } else {
        Err(ConfigError::format(format_args!(
            "{} must be greater than zero, but {} given",
            name, value
        )))
    }
}
