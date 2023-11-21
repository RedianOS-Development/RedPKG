use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
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
    pkgs: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Remove {
    pkgs: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Search {
    terms: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Query {
    terms: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct AddRepo {
    repo: String,
}
