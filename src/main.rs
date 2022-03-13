use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tama::{
    error::Result,
    host_config::{get_host_config, HostConfig},
    tomcat::list,
};

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
    fn handle(self, config: &HostConfig) -> i32 {
        match self {
            MainAction::List => {
                list(config);
                0
            }
            _ => 0,
        }
    }
}

fn handle_error<T>(r: Result<T>, error_exit: i32) -> T {
    match r {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            std::process::exit(error_exit)
        }
    }
}

fn main() {
    let config = handle_error(get_host_config(), 1);
    let result = Cli::parse().action.handle(&config);
    std::process::exit(result);
}
