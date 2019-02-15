use super::ConfigError;
use super::ConfigRef;
use super::ConfigResult;

#[allow(clippy::needless_pass_by_value)]
pub fn validate(config: ConfigRef) -> ConfigResult<()> {
    validate_number(config.update_interval(), "update_interval")?;

    for server in config.servers() {
        validate_number(server.disk_capacity(), "disk_capacity")?;
        validate_number(server.soft_threshold(), "soft_threshold")?;
        validate_number(server.hard_threshold(), "hard_threshold")?;
    }

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
