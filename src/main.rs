mod cli;

use clap::Parser;
use colored::Colorize;

fn main() {
    let args = cli::Cli::parse();
}
