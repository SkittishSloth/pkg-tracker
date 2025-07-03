use crate::Args;

pub fn run(args: Args) {
    if args.verbose > 0 {
        eprintln!("Updating package repo....");
    }
}