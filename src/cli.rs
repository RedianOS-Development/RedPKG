use clap::Clap;

#[derive(Clap)]
/// RedPKG: RedianOS's Native Package Manager
pub struct Cli {
    #[clap(subcommand)]
    pub operation: Operations,
}

#[derive(Debug, Clap)]
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

#[derive(Debug, Clap)]
pub struct Install {
    #[clap(index = 1)]
    pub pkgs: Vec<String>,
}

#[derive(Debug, Clap)]
pub struct Remove {
    #[clap(index = 1)]
    pub pkgs: Vec<String>,
}

#[derive(Debug, Clap)]
pub struct Search {
    #[clap(index = 1)]
    pub terms: Vec<String>,
}

#[derive(Debug, Clap)]
pub struct Query {
    #[clap(index = 1)]
    pub terms: Vec<String>,
}

#[derive(Debug, Clap)]
pub struct AddRepo {
    #[clap(index = 1)]
    pub repo: String,
}

