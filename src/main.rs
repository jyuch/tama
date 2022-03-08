use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

fn main() {
    Cli::parse();
}
