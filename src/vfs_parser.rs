use vfs::{MemoryFS, VfsPath, VfsResult};

pub struct VfsParser {
    root_vfs: VfsPath,
}

impl VfsParser {
    pub fn new() -> Self {
        VfsParser {
            root_vfs: VfsPath::new(MemoryFS::new()),
        }
    }

    pub fn parse(&mut self, path: String) {
        // TODO: Parse comma-separated files/folders at subfolder.
        let dir = self.root_vfs.join(path).unwrap();
        dir.create_dir_all().unwrap();
    }

    pub fn print_tree(self) {
        let directories = self
            .root_vfs
            .walk_dir()
            .unwrap()
            .collect::<VfsResult<Vec<_>>>()
            .unwrap();

        for d in directories {
            println!("{:?}", d);
        }
    }
}
