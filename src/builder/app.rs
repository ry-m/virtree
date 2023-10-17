use std::io;

use crate::vfs_parser::VfsParser;

pub(crate) struct App<'a> {
    parser: &'a VfsParser
}

impl<'a> App<'a> {
    pub fn new(parser: &'a VfsParser) -> Self {
        Self {
            parser,
        }
    }

    pub fn run(&self) -> io::Result<()> {
        self.parser.print_tree();
        Ok(())
    }
}