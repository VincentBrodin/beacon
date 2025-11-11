use std::{cmp, env, io, sync::Arc, time::Instant};

use memory_stats::memory_stats;
use nucleo::{
    Config, Nucleo,
    pattern::{CaseMatching, Normalization},
};
use pretty_bytes::converter::convert;
use thiserror::Error;

use crate::{app::App, desktop::Desktop};

mod app;
mod desktop;

#[derive(Error, Debug)]
enum Error {
    #[error("Eframe failed: {0}")]
    Eframe(#[from] eframe::Error),
    #[error("Io failed: {0}")]
    IO(#[from] io::Error),
    #[error("Failed to parse ini: {0}")]
    Ini(#[from] ini::ParseError),
}

fn main() -> Result<(), Error> {
    let mut args = env::args();
    let _ = args.next();
    let search_str = match args.next() {
        Some(val) => val,
        None => String::from(""),
    };

    // For future use
    let notify = Arc::new(|| {});

    let config = Config::DEFAULT;
    let mut nucleo: Nucleo<Desktop> = Nucleo::new(config, notify, None, 1);

    let mut now = Instant::now();
    let injector = nucleo.injector();
    Desktop::for_each_in_directory("/usr/share/applications", |desktop| {
        injector.push(desktop, |desktop, column| {
            column[0] = nucleo::Utf32String::Unicode(desktop.chars.clone().into_boxed_slice());
        });
    })?;
    let mut duration = now.elapsed();
    println!("Took {:?} to inject apps", duration);

    // let searcher = Searcher::new(apps, config);
    now = Instant::now();
    nucleo.pattern.reparse(
        0,
        &search_str,
        CaseMatching::Ignore,
        Normalization::Smart,
        false,
    );
    nucleo.tick(10);
    let snapshot = nucleo.snapshot();
    duration = now.elapsed();
    println!("Matched and sorted for {} in {:?}", search_str, duration);

    for item in snapshot.matched_items(0..cmp::min(5, snapshot.matched_item_count())) {
        println!("{}", item.data.name);
    }
    if let Some(usage) = memory_stats() {
        println!(
            "Current physical memory usage: {}",
            convert(usage.physical_mem as f64)
        );
        println!(
            "Current virtual memory usage: {}",
            convert(usage.virtual_mem as f64)
        );
    } else {
        println!("Couldn't get the current memory usage :(");
    }

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
        Box::new(|_| Ok(Box::new(App::new(nucleo)))),
    )?;
    Ok(())
}
