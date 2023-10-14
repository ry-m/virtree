mod builder;
mod vfs_parser;

use builder::Builder;

use clap::Parser;
use vfs_parser::VfsParser;

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
        let mut parser = VfsParser::new(); 
        for item in cli.input {
            parser.parse_item(item);
        }

        parser.print_tree();
    }
}
