use std::{collections::HashMap, fs::OpenOptions};

use crate::{Error, config::config_dir};

pub const HISTORY_FILE: &str = "history.json";

pub fn load_from_file() -> Result<HashMap<String, usize>, Error> {
    let config_dir = config_dir()?;
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open(config_dir.join(HISTORY_FILE))?;
    let result: HashMap<String, usize> = serde_json::from_reader(file).unwrap_or_default();
    Ok(result)
}

pub fn write_to_file(history: &HashMap<String, usize>) -> Result<(), Error> {
    let config_dir = config_dir()?;
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open(config_dir.join(HISTORY_FILE))?;
    serde_json::to_writer(file, history)?;
    Ok(())
}
