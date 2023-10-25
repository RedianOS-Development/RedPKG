use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(about, long_about)]
/// RedPKG: RedianOS's Native Package Manager
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
    #[clap(index = 1)]
    pub pkgs: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Remove {
    #[clap(index = 1)]
    pub pkgs: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Search {
    #[clap(index = 1)]
    pub terms: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Query {
    #[clap(index = 1)]
    pub terms: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct AddRepo {
    #[clap(index = 1)]
    pub repo: String,
}
