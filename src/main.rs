use std::{io, sync::Arc, time::Instant};

use nucleo::{Config, Nucleo};
use thiserror::Error;

use crate::{app::App, desktop::Desktop};

mod app;
mod config;
mod desktop;
mod history;

pub const APP_NAME: &str = "beacon";

#[derive(Error, Debug)]
enum Error {
    #[error("Eframe failed: {0}")]
    Eframe(#[from] eframe::Error),
    #[error("Io failed: {0}")]
    IO(#[from] io::Error),
    #[error("Failed to parse ini: {0}")]
    Ini(#[from] ini::ParseError),
    #[error("Serde failed: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Failed to get config directory")]
    ConfigDirectory,
}

fn main() -> Result<(), Error> {
    // Setup nucleo
    let mut now = Instant::now();
    let notify = Arc::new(|| {});
    let config = Config::DEFAULT;
    let nucleo: Nucleo<Desktop> = Nucleo::new(config, notify, None, 1);
    let injector = nucleo.injector();
    Desktop::for_each_in_directory("/usr/share/applications", |desktop| {
        injector.push(desktop, |desktop, column| {
            column[0] = nucleo::Utf32String::Unicode(desktop.boxed_chars.clone());
        });
    })?;
    let mut duration = now.elapsed();
    println!("Took {:?} to inject apps", duration);

    // Load history
    now = Instant::now();
    let history = history::load_from_file()?;
    duration = now.elapsed();
    println!("Took {:?} to load history", duration);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_title("beacon")
            .with_decorations(false)
            .with_resizable(false)
            .with_transparent(false)
            .with_window_level(egui::WindowLevel::AlwaysOnTop),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|_| Ok(Box::new(App::new(nucleo, history)))),
    )?;
    Ok(())
}
