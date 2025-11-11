use std::{fs, path::PathBuf};

use crate::{APP_NAME, Error};

pub fn config_dir() -> Result<PathBuf, Error> {
    if let Some(config) = dirs::config_dir() {
        let app_config = config.join(APP_NAME);
        if !app_config.exists() {
            fs::create_dir_all(app_config.clone())?
        }
        Ok(app_config)
    } else {
        Err(Error::ConfigDirectory)
    }
}
