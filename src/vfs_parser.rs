use std::borrow::Cow;

use anyhow::{Context, Result};
use ptree::TreeItem;
use vfs::{MemoryFS, VfsPath};

/// VfsParser is a struct wrapper for a VfsPath. It implements ptree::TreeItem for tree
/// printing along with parsing-related functions.
#[derive(Clone)]
pub struct VfsParser(VfsPath);

/// Implementation of TreeItem for VfsParser, allowing displaying the tree.
impl TreeItem for VfsParser {
    type Child = Self;

    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &ptree::Style,
    ) -> std::io::Result<()> {
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
        let mut v = if let Ok(list) = self.0.read_dir() {
            list.map(VfsParser).collect()
        } else {
            Vec::new()
        };

        // Sort the children by alphabetical order.
        // In the future, this can be changed to a different order.
        v.sort_by(|a, b| {
            a.0.as_str()
                .partial_cmp(b.0.as_str())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Cow::from(v)
    }
}

impl VfsParser {
    pub fn new() -> Self {
        VfsParser(VfsPath::new(MemoryFS::new()))
    }

    /// Parses a single directory or file item from its path
    pub fn parse_item(&mut self, input: String) -> Result<()> {
        // First, trim the end separator (if it exists).
        let trimmed = input.trim_end_matches(VfsParser::is_path_separator);
        // Split the last path separated item from its parent directory.
        let split = trimmed.rsplit_once(VfsParser::is_path_separator);

        // If the split was successful (implies there was a nested directory), check
        // for comma separated values on the child item.
        if let Some((parent_path, subpaths)) = split {
            for sp in subpaths.split(",") {
                // Join the root, parent, and child paths. Append all.
                let vpath = self
                    .0
                    .join(parent_path)
                    .with_context(|| format!("could not join parent path to root"))?
                    .join(sp)
                    .with_context(|| format!(""))?;

                vpath.create_dir_all().with_context(|| format!(""))?;
            }
        } else {
            // Singular item
            let dir = self
                .0
                .join(input)
                .with_context(|| format!("could not join path to root directory"))?;

            dir.create_dir_all()
                .with_context(|| format!("could not create directories"))?;
        }

        Ok(())
    }

    /// Prints out the directory tree using the ptree library.
    pub fn print_tree(self) {
        ptree::output::print_tree(&self).unwrap();
    }

    /// Returns true if c is a path separator
    fn is_path_separator(c: char) -> bool {
        c == '/' || c == '\\'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_singulars() {
        let mut parser = VfsParser::new();
        for x in ["foo", "bar", "abc"] {
            parser.parse_item(x.to_string()).unwrap();
        }

        for path in parser.0.read_dir().unwrap() {
            assert!(&["/foo", "/bar", "/abc"].contains(&path.as_str()));
        }
    }

    #[test]
    fn test_parse_subdirs() {
        let mut parser = VfsParser::new();
        for x in ["foo", "foo/bar", "foo/abc", "foo/bar/xyz"] {
            parser.parse_item(x.to_string()).unwrap();
        }

        for path in parser.0.read_dir().unwrap() {
            assert!(&["/foo", "/foo/bar", "/foo/abc", "/foo/bar/xyz"].contains(&path.as_str()));
        }
    }

    #[test]
    fn test_multidirs() {
        let mut parser = VfsParser::new();
        for x in ["foo/a,b,c", "foo/a/x,y"] {
            parser.parse_item(x.to_string()).unwrap();
        }

        for path in parser.0.read_dir().unwrap() {
            assert!(
                &["/foo", "/foo/a", "/foo/b", "/foo/c", "/foo/a/x", "/foo/a/y"]
                    .contains(&path.as_str())
            );
        }
    }
}
