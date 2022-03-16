use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tama::error::Response;
use tama::tomcat::{deploy, undeploy};
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
    fn handle(self, config: &HostConfig) -> Result<Response> {
        match self {
            MainAction::List => list(config),
            MainAction::Deploy { context, war } => deploy(config, &context, &war),
            MainAction::Undeploy { context } => undeploy(config, &context),
            _ => unimplemented!(),
        }
    }
}

fn handle_error<T>(r: Result<T>) -> T {
    match r {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    }
}

fn main() {
    let config = get_host_config();
    let config = handle_error(config);

    let result: Result<Response> = Cli::parse().action.handle(&config);
    let response = handle_error(result);

    match response {
        tama::error::Response::Ok(Some(text)) => {
            println!("OK - {}", text);
            std::process::exit(0)
        }
        tama::error::Response::Ok(None) => std::process::exit(0),
        tama::error::Response::Fail(Some(text)) => {
            println!("FAIL - {}", text);
            std::process::exit(1)
        }
        tama::error::Response::Fail(None) => std::process::exit(1),
    }
}
