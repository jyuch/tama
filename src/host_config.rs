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
    HostError,
    #[error("TOMCAT_HOST is malformed")]
    MalformedUrlError,
    #[error("TOMCAT_USER not set")]
    UserNameError,
    #[error("TOMCAT_PASSWORD not set")]
    PasswordError,
}

pub fn get_host_config() -> Result<HostConfig> {
    let host = std::env::var("TOMCAT_HOST").map_err(|_| HostConfigError::HostError)?;
    let user_name = std::env::var("TOMCAT_USER").map_err(|_| HostConfigError::UserNameError)?;
    let password = std::env::var("TOMCAT_PASSWORD").map_err(|_| HostConfigError::PasswordError)?;
    let host = Url::parse(&host).map_err(|_| HostConfigError::MalformedUrlError)?;
    Ok(HostConfig::new(host, user_name, password))
}
