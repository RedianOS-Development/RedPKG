mod cli;
mod log;

use clap::{App, Arg, Subcommand}; // Added missing import for Subcommand

fn main() {
    let matches = App::new("RedPKG")
        .version("1.0")
        .author("Oglo12 , Rudy ツ")
        .about("RedianOS's Native Package Manager")
        .subcommand(Subcommand::with_name("install")
            .about("Install packages")
            .arg(Arg::with_name("package")
                .required(true)
                .takes_value(true)))
        .subcommand(Subcommand::with_name("remove")
            .about("Uninstall packages")
            .arg(Arg::with_name("package")
                .required(true)
                .takes_value(true)))
        .subcommand(Subcommand::with_name("search")
            .about("Search repositories for packages")
            .arg(Arg::with_name("package")
                .required(true)
                .takes_value(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        let package = matches.value_of("package").unwrap();
        // Handle install command with the provided package name
        println!("Installing package: {}", package);
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let package = matches.value_of("package").unwrap();
        // Handle remove command with the provided package name
        println!("Removing package: {}", package);
    } else if let Some(matches) = matches.subcommand_matches("search") {
        let package = matches.value_of("package").unwrap();
        // Handle search command with the provided package name
        println!("Searching for package: {}", package);
    } else {
        println!("Invalid command");
    }
}
