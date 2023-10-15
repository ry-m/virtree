mod builder;
mod vfs_parser;

use builder::Builder;

use anyhow::{Context, Result};
use clap::Parser;
use vfs_parser::VfsParser;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Include the actual directory tree from where virtree is executed from.
    #[clap(long, action)]
    include: bool,

    /// Sequence of virtual file and folder names.
    input: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut parser = VfsParser::new();

    // Check the --include flag.
    // If enabled, include the contents of the current directory in the virtual file system.
    if cli.include {
        for entry in WalkDir::new(".") {
            match entry {
                Ok(entry) => {
                    parser
                        .parse_item(entry.path().to_string_lossy().to_string())
                        .with_context(|| format!("path error"))?;
                }
                Err(err) => {
                    eprintln!("warning: {}", err);
                }
            }
        }
    }

    if cli.input.len() == 0 {
        // Run builder mode
        Builder::new(parser).run();
    } else {
        for item in cli.input {
            parser.parse_item(item)?;
        }

        parser.print_tree();
    }

    Ok(())
}
