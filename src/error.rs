use custom_error::custom_error;
use std::io;

custom_error! {pub AppError
    Io{source: io::Error}         = "unable to read from the file",
    Config{source: config::ConfigError} = "invalid config",
    Environment{env:String} = "unknown environment {env}",

}
