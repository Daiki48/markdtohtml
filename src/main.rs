use markdtohtml::lexical;
use std::fs::{read_dir, DirEntry};
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    for entry in read_dir(".")? {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();
        lexical(path)?;
    }
    Ok(())
}
