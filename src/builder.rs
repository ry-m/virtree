use crate::vfs_parser::VfsParser;

pub struct Builder {
    _parser: VfsParser,
}

impl Builder {
    pub fn new(parser: VfsParser) -> Self {
        Builder {
            _parser: parser,
        }
    }

    pub fn run(self) {}
}
