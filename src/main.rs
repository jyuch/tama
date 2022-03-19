use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tama::error::Response;
use tama::tomcat::{deploy, reload, start, stop, undeploy};
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
        context_path: String,
        /// Path to war file.
        #[clap(long, short)]
        war_file: PathBuf,
    },
    /// Undeploy an existing application.
    Undeploy {
        /// Context path.
        #[clap(long, short)]
        context_path: String,
    },
    /// List currently deployed applications.
    List,
    /// Reload an existing application.
    Reload {
        /// Context path.
        #[clap(long, short)]
        context_path: String,
    },
    /// Start an existing application.
    Start {
        /// Context path.
        #[clap(long, short)]
        context_path: String,
    },
    /// Stop an existing application.
    Stop {
        /// Context path.
        #[clap(long, short)]
        context_path: String,
    },
}

impl MainAction {
    fn handle(self, config: &HostConfig) -> Result<Response> {
        match self {
            MainAction::Deploy {
                context_path,
                war_file,
            } => deploy(config, &context_path, &war_file),
            MainAction::Undeploy { context_path } => undeploy(config, &context_path),
            MainAction::List => list(config),
            MainAction::Reload { context_path } => reload(config, &context_path),
            MainAction::Start { context_path } => start(config, &context_path),
            MainAction::Stop { context_path } => stop(config, &context_path),
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
    let opt = Cli::parse();

    let config = get_host_config();
    let config = handle_error(config);

    let result: Result<Response> = opt.action.handle(&config);
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
