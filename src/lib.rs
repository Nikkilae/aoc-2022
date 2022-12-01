use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    iter::once,
    path::Path,
};

pub fn read_lines<P: AsRef<Path>>(
    relative_path: P,
) -> Result<impl Iterator<Item = String>, Box<dyn Error>> {
    let full_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    let file = File::open(full_path)?;
    let it = BufReader::new(file)
        .lines()
        .flatten()
        .chain(once("".to_string()));
    Ok(it)
}
