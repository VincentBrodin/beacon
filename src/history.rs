use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self},
    path::Path,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
}
pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<HashMap<String, usize>, Error> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open(&path)?;
    let result: HashMap<String, usize> = serde_json::from_reader(file).unwrap_or_default();
    Ok(result)
}

pub fn write_to_file<P: AsRef<Path>>(
    path: P,
    history: &HashMap<String, usize>,
) -> Result<(), Error> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open(&path)?;
    serde_json::to_writer(file, history)?;
    Ok(())
}
