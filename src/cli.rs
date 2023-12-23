// cli.rs

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
/// RedPKG : RedianOS's Native Package Manager
pub struct Cli {
    #[clap(subcommand)]
    pub operation: Operations,
}

#[derive(Debug, Subcommand)]
/// What shall RedPKG do for you?
pub enum Operations {
    /// Install packages
    Install(Install),

    /// Uninstall packages
    Remove(Remove),

    /// Search repositories for package
    Search(Search),

    /// Search system for installed package
    Query(Query),

    /// List all installed packages
    List,

    /// Upgrade system packages
    Upgrade,

    /// Sync repositories
    Sync,

    /// Add repository
    AddRepo(AddRepo),
}

#[derive(Parser, Debug)]
pub struct Install {
    #[clap(name = "packages")]
    pub pkgs: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Remove {
    #[clap(name = "packages")]
    pub pkgs: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Search {
    #[clap(name = "search-terms")]
    pub terms: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Query {
    #[clap(name = "query-terms")]
    pub terms: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct AddRepo {
    #[clap(name = "repository")]
    pub repo: String,
}
