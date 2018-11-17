use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct Endpoint {
    url: String,
}

impl Endpoint {
    pub fn url(&self) -> &String {
        &self.url
    }
}


#[derive(Debug, Deserialize)]
pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    endpoint: Endpoint,
    server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        settings.merge(File::with_name(&format!("config/env.toml")).required(true))?;

        settings.try_into()
    }

    pub fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}
