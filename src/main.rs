mod builder;

use builder::Builder;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // #[clap(long, action)]
    // it_just_works: bool,

    /// Sequence of virtual file and folder names.
    input: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.input.len() == 0 {
        // Run builder mode
        Builder::new().run();
    } else {
        // Parse all input and build the tree immediately
        for input in cli.input {
            dbg!(input);
        }
    }
}
