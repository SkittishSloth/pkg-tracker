use clap::Command;

fn main() {
    let matches = Command::new("Brew Plugin")
        .version("1.0")
        .author("Your Name <your@email.com>")
        .about("A plugin for managing Homebrew packages")
        .subcommand(Command::new("list")
            .about("Lists installed packages"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        println!("Listing packages!");
    }
}
