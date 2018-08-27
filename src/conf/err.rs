use failure::Fail;

#[derive(Fail, Debug)]
pub enum ConfigurationError {
    #[fail(display = "Could not find home directory. Try setting the $PATH variable")]
    CouldNotFindHomeDir,
    #[fail(display = "Invalid Toml: {}", _0)]
    InvalidToml(#[fail(cause)] toml::de::Error),
    #[fail(display = "Error serializing configuration: {}", _0)]
    DecodeError(#[fail(cause)] toml::ser::Error),
    #[fail(display = "Input/Output Error")]
    IOError(#[fail(cause)] std::io::Error),
    #[fail(display = "{} not found!", _0)]
    NotFound(String),
    #[fail(display = "Invalid path for config file; not a valid UTF-8 String!")]
    InvalidConfigPath,
    #[fail(display = "Option Not Set: {}", _0)]
    OptionNotSet(String),
    #[fail(display = "Generation failed; Default configuration file already exists!")]
    ConfigExists,
    #[fail(display = "Config Error occurred")]
    Config(#[cause] config::ConfigError),
}

impl From<config::ConfigError> for ConfigurationError {
    fn from(err: config::ConfigError) -> ConfigurationError {
        ConfigurationError::Config(err)
    }
}

impl From<std::io::Error> for ConfigurationError {
    fn from(err: std::io::Error) -> ConfigurationError {
        ConfigurationError::IOError(err)
    }
}

impl From<toml::de::Error> for ConfigurationError {
    fn from(err: toml::de::Error) -> ConfigurationError {
        ConfigurationError::InvalidToml(err)
    }
}

impl From<toml::ser::Error> for ConfigurationError {
    fn from(err: toml::ser::Error) -> ConfigurationError {
        ConfigurationError::DecodeError(err)
    }
}
