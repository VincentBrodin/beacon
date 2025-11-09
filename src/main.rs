use std::{cmp::min, env, fs, io, path::Path, time::Instant};

use nucleo_matcher::Config;
use rayon::prelude::*;
use thiserror::Error;

use crate::{application::Application, searcher::Searcher};

mod application;
mod searcher;

#[derive(Error, Debug)]
enum Error {
    #[error("Iced failed: {0}")]
    Iced(#[from] iced::Error),
    #[error("Io failed: {0}")]
    IO(#[from] io::Error),
    #[error("Failed to parse ini: {0}")]
    Ini(#[from] ini::ParseError),
    #[error("Missing application folder")]
    MissingAppFolder,
    #[error("Missing argument")]
    MissingArg,
}

fn main() -> Result<(), Error> {
    let mut args = env::args();
    let _ = args.next();
    let search_str = match args.next() {
        Some(val) => val,
        None => return Err(Error::MissingArg),
    };

    let mut now = Instant::now();
    let app_folder = Path::new("/usr/share/applications");
    if !app_folder.exists() {
        return Err(Error::MissingAppFolder);
    }
    let dir_entries: Vec<_> = fs::read_dir(app_folder)?
        .filter_map(|entry| entry.ok())
        .collect();

    let apps: Vec<_> = dir_entries
        .par_iter()
        .filter_map(|entry| Application::load_from_file(entry.path()))
        .filter(|app| !app.no_display)
        .collect();

    let mut duration = now.elapsed();
    print!("Took {:?} to load {} apps\n", duration, apps.len());

    let mut config = Config::DEFAULT;
    config.prefer_prefix = true;

    let searcher = Searcher::new(apps, config);
    now = Instant::now();
    let results = searcher.search(&search_str);
    duration = now.elapsed();
    print!("Matched and sorted for {} in {:?}\n", search_str, duration);

    for result in &results[0..min(5, results.len())] {
        print!("Found match for {}!\n", result.name);
    }

    Ok(())
}
