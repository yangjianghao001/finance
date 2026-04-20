mod server;

use config::Config;
use serde::Deserialize;
use std::sync::LazyLock;

static APPLICATION_CONFIG: LazyLock<Application> =
    LazyLock::new(|| Application::load().expect("Failed to load application.yaml"));
#[derive(Debug, Deserialize)]
pub struct Application {
    pub server: server::Server,
}

impl Application {
    fn load() -> Result<Self, config::ConfigError> {
        let application_yaml = include_str!("../../../../resource/application.yaml");
        Config::builder()
            .add_source(config::File::from_str(
                application_yaml,
                config::FileFormat::Yaml,
            ))
            .build()?
            .try_deserialize()
    }
}

pub fn get() -> &'static Application {
    &APPLICATION_CONFIG
}
