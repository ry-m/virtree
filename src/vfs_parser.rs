use std::borrow::Cow;

use ptree::TreeItem;
use vfs::{MemoryFS, VfsPath};

/// VfsParser is a struct wrapper for a VfsPath. It implements ptree::TreeItem for tree 
/// printing along with parsing-related functions. 
#[derive(Clone)]
pub struct VfsParser(VfsPath);

/// Implementation of TreeItem for VfsParser, allowing displaying the tree.
impl TreeItem for VfsParser {
    type Child = Self;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &ptree::Style) -> std::io::Result<()> {
        let display_name = if self.0.filename().is_empty() {
            // Display name is empty, use a period to denote root and/or empty. 
            // Similar to *nix tree program. 
            ".".to_string()
        } else {
            // Use the file/directory name. 
            self.0.filename()
        };

        write!(f, "{}", style.paint(display_name))
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        let v = if let Ok(list) = self.0.read_dir() {
            list.map(VfsParser).collect()
        } else {
            Vec::new()
        };

        Cow::from(v)
    } 
}

impl VfsParser {
    pub fn new() -> Self {
        VfsParser(VfsPath::new(MemoryFS::new()))
    }

    pub fn parse_item(&mut self, input: String) {
        // TODO: Fix use of unwrap()

        // First, trim the end separator (if it exists). 
        let trimmed = input.trim_end_matches(VfsParser::is_path_separator);
        // Split the last path separated item from its parent directory. 
        let split = trimmed.rsplit_once(VfsParser::is_path_separator);

        // If the split was successful (implies there was a nested directory), check
        // for comma separated values on the child item. 
        if let Some((parent_path, subpaths)) = split {
            for sp in subpaths.split(",") {
                // Join the root, parent, and child paths. Append all.
                let vpath = self.0.join(parent_path).unwrap().join(sp).unwrap();
                vpath.create_dir_all().unwrap();
            }
        } else {
            // Singular item
            let dir = self.0.join(input).unwrap();
            dir.create_dir_all().unwrap();
        }
    }

    pub fn print_tree(self) {
        ptree::output::print_tree(&self).unwrap();
    }

    fn is_path_separator(c: char) -> bool {
        c == '/' || c == '\\'
    }
}
