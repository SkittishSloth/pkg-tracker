use clap::Parser;

mod tracker;

#[derive(Debug, Parser)]
#[command(
    name = "pkg-tracker-termux", 
    version = "0.1.0", 
    about = "A plugin for managing Termux packages", 
    long_about = None
)]
struct Args {
    /// Set the level of verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();

    if args.verbose > 0 {
        eprintln!("Verbose mode enabled (level {})", args.verbose);
    }

    tracker::run(args);
}