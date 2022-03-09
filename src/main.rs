use std::io::Write;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tama::credential::add_credential;
use crate::CredentialAction::Add;
use crate::MainAction::Credential;

#[derive(Parser, Debug)]
#[clap(bin_name = "tama")]
#[clap(version, about)]
struct Cli {
    #[clap(subcommand)]
    action: MainAction,
}

#[derive(Subcommand, Debug)]
enum MainAction {
    /// Manage credentials to use tomcat management.
    Credential {
        #[clap(subcommand)]
        action: CredentialAction,
    },
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

#[derive(Subcommand, Debug)]
enum CredentialAction {
    /// Add new credential to credential manager.
    Add {
        /// Tomcat manager base url.
        url: String,
        /// Tomcat user name.
        user: String,
    },
    /// Remove specified credential from credential manager.
    Remove {
        /// Tomcat manager base url.
        url: String,
        /// Tomcat user name.
        user: String,
    },
}

impl MainAction {
    fn handle(self) -> i32 {
        match self {
            Credential { action } => {
                MainAction::handle_result(action.handle())
            }
            _ => unimplemented!(),
        }
    }

    fn handle_result(result: Result<(), Box<dyn std::error::Error>>) -> i32 {
        match result {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("{}", e);
                1
            }
        }
    }
}

impl CredentialAction {
    fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Add { url, user } => {
                let mut buf = String::new();
                print!("> ");
                std::io::stdout().flush()?;
                std::io::stdin().read_line(&mut buf)?;
                let password = buf.trim();
                let _ = add_credential(&url, &user, password)?;
                Ok(())
            }
            _ => unimplemented!()
        }
    }
}

fn main() {
    let result = Cli::parse().action.handle();
    std::process::exit(result);
}
