use ropey::{Rope, RopeBuilder};
use std::{fs::File, path::PathBuf};

pub fn open_file_with_rope(file: PathBuf) -> Rope {
    let text = Rope::from_reader(File::open(file).unwrap()).unwrap();
    text
}

//* Using rope science for advanced text manipulation. */
