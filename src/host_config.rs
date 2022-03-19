use crate::error::Result;
use reqwest::Url;
use thiserror::Error;

#[derive(Debug)]
pub struct HostConfig {
    pub host: Url,
    pub user_name: String,
    pub password: String,
}

impl HostConfig {
    pub fn new(host: Url, user_name: String, password: String) -> HostConfig {
        HostConfig {
            host,
            user_name,
            password,
        }
    }
}

#[derive(Debug, Error)]
enum HostConfigError {
    #[error("TOMCAT_HOST not set")]
    HostNotSet,
    #[error("TOMCAT_HOST is malformed")]
    MalformedHostUrl,
    #[error("TOMCAT_USER not set")]
    UserNameNotSet,
    #[error("TOMCAT_PASSWORD not set")]
    PasswordNotSet,
}

pub fn get_host_config() -> Result<HostConfig> {
    let host = std::env::var("TOMCAT_HOST").map_err(|_| HostConfigError::HostNotSet)?;
    let user_name = std::env::var("TOMCAT_USER").map_err(|_| HostConfigError::UserNameNotSet)?;
    let password = std::env::var("TOMCAT_PASSWORD").map_err(|_| HostConfigError::PasswordNotSet)?;
    let host = Url::parse(&host).map_err(|_| HostConfigError::MalformedHostUrl)?;
    Ok(HostConfig::new(host, user_name, password))
}
