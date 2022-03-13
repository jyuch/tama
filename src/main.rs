use clap::{Parser, Subcommand};
use std::io::Write;
use std::path::PathBuf;
use thiserror::Error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

fn handle_error<T>(r: Result<T>) -> T {
    match r {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    }
}

#[derive(Parser, Debug)]
#[clap(bin_name = "tama")]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    action: MainAction,
}

#[derive(Subcommand, Debug)]
enum MainAction {
    /// Deploy a new application.
    Deploy {
        /// Context path.
        #[clap(long, short)]
        context: String,
        /// Path to war file.
        #[clap(long, short)]
        war: PathBuf,
    },
    /// Undeploy an existing application.
    Undeploy {
        /// Context path.
        #[clap(long, short)]
        context: String,
    },
    /// List currently deployed applications.
    List,
    /// Reload an existing application.
    Reload {
        /// Context path.
        #[clap(long, short)]
        context: String,
    },
    /// Start an existing application.
    Start {
        /// Context path.
        #[clap(long, short)]
        context: String,
    },
    /// Stop an existing application.
    Stop {
        /// Context path.
        #[clap(long, short)]
        context: String,
    },
}

impl MainAction {
    fn handle(self) -> i32 {
        match self {
            _ => unimplemented!(),
        }
    }

    fn handle_result(result: Result<()>) -> i32 {
        match result {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("{}", e);
                1
            }
        }
    }
}

#[derive(Debug, Error)]
enum HostConfigError {
    #[error("TOMCAT_HOST not set")]
    HostError,
    #[error("TOMCAT_USER not set")]
    UserNameError,
    #[error("TOMCAT_PASSWORD not set")]
    PasswordError,
}

#[derive(Debug)]
struct HostConfig {
    host: String,
    user_name: String,
    password: String,
}

impl HostConfig {
    fn new(host: String, user_name: String, password: String) -> HostConfig {
        HostConfig {
            host,
            user_name,
            password,
        }
    }
}

fn get_host_config() -> Result<HostConfig> {
    let host = std::env::var("TOMCAT_HOST").map_err(|_| HostConfigError::HostError)?;
    let user_name = std::env::var("TOMCAT_USER").map_err(|_| HostConfigError::UserNameError)?;
    let password = std::env::var("TOMCAT_PASSWORD").map_err(|_| HostConfigError::PasswordError)?;
    Ok(HostConfig::new(host, user_name, password))
}

fn main() {
    let config = handle_error(get_host_config());
    println!("{:?}", config);
    let result = Cli::parse().action.handle();
    std::process::exit(result);
}
