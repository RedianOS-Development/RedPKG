mod cli;
mod log;

use clap::Clap;

#[derive(Clap)]
#[clap(author, version, about, long_about)]
/// RedPKG: RedianOS's Native Package Manager
struct Opts {
    #[clap(subcommand)]
    pub operation: cli::Operations,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.operation {
        cli::Operations::Install(install) => {
            // Handle install command with the provided package names
            println!("Installing packages: {:?}", install.pkgs);
        }
        cli::Operations::Remove(remove) => {
            // Handle remove command with the provided package names
            println!("Removing packages: {:?}", remove.pkgs);
        }
        cli::Operations::Search(search) => {
            // Handle search command with the provided search terms
            println!("Searching packages: {:?}", search.terms);
        }
        cli::Operations::Query(query) => {
            // Handle query command with the provided query terms
            println!("Querying packages: {:?}", query.terms);
        }
        cli::Operations::List => {
            // Handle list command
            println!("Listing packages");
        }
        cli::Operations::Upgrade => {
            // Handle upgrade command
            println!("Upgrading system packages");
        }
        cli::Operations::Sync => {
            // Handle sync command
            println!("Syncing repositories");
        }
        cli::Operations::AddRepo(add_repo) => {
            // Handle add repo command with the provided repository
            println!("Adding repository: {}", add_repo.repo);
        }
    }
}

