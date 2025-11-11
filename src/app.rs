use std::{cmp, collections::HashMap, time::Instant};

use egui::{Align, CentralPanel, Key, Layout, ScrollArea, TextEdit, ViewportCommand};
use nucleo::Nucleo;

use crate::{HISTORY, config::config_dir, desktop::Desktop, history};

pub struct App {
    /// Maximum amount of visible items at one time
    pub max_items: usize,

    /// Current query
    query: String,
    /// What the query was last frame
    last_query: String,
    /// Index of the currently selected item
    selected: usize,
    nucleo: Nucleo<Desktop>,
    history: HashMap<String, usize>,
    /// When this is set to true, the window will close the next frame
    close: bool,
    list: Vec<String>,
}

impl App {
    pub fn new(nucleo: Nucleo<Desktop>, history: HashMap<String, usize>) -> Self {
        Self {
            query: String::from(""),
            last_query: String::from(""),
            selected: 0,
            max_items: 50,
            nucleo,
            history,
            close: false,
            list: Vec::new(),
        }
    }

    fn select_up(&mut self) {
        let snapshot = self.nucleo.snapshot();
        let count = cmp::min(snapshot.matched_item_count() as usize, self.max_items);
        if self.selected == 0 {
            self.selected = count - 1;
        } else {
            self.selected -= 1;
        }
    }
    fn select_down(&mut self) {
        let snapshot = self.nucleo.snapshot();
        let count = cmp::min(snapshot.matched_item_count() as usize, self.max_items);

        if self.selected + 1 >= count {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }

    fn select(&mut self) {
        let snapshot = self.nucleo.snapshot();
        let item = snapshot.get_matched_item(self.selected as u32);
        if let Some(item) = item {
            let count = *self.history.get(&item.data.name).unwrap_or(&0) + 1;
            self.history.insert(item.data.name.clone(), count);
            if let Ok(config_dir) = config_dir() {
                let _ = history::write_to_file(config_dir.join(HISTORY), &self.history)
                    .map_err(|err| println!("Failed to save history: {}", err));
            }
            println!("{} got a score of {}", item.data.name, count);
            let _ = item.data.start().map_err(|err| {
                println!("Failed to run {}: {}", item.data.name, err);
            });
        } else {
            println!("No item found")
        }
        self.close = true; // Close next frame
    }

    fn update_list(&mut self) {
        let now = Instant::now();
        let snapshot = self.nucleo.snapshot();
        let matched_count = snapshot.matched_item_count() as usize;
        let max_count = cmp::min(matched_count, self.max_items);
        let items_with_count: Vec<_> = snapshot
            .matched_items(..)
            .map(|item| {
                let count = *self.history.get(&item.data.name).unwrap_or(&0);
                (item.data.name.as_str(), count)
            })
            .collect();

        let mut sorted_items: Vec<_> = items_with_count
            .iter()
            .enumerate()
            .map(|(index, (name, count))| {
                let value = match snapshot.pattern().is_empty() {
                    true => *count,
                    false => matched_count - index + (*count * 5),
                };
                (name, value)
            })
            .collect();

        sorted_items.sort_by(|a, b| b.1.cmp(&a.1));
        self.list = sorted_items
            .iter()
            .take(max_count)
            .map(|(item, _)| item.to_string())
            .collect();
        let duration = now.elapsed();
        println!("Took {:?} to update list", duration);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.close {
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }
        if self.last_query != self.query {
            self.last_query = self.query.clone();
            self.nucleo.pattern.reparse(
                0,
                &self.query,
                nucleo::pattern::CaseMatching::Smart,
                nucleo::pattern::Normalization::Smart,
                false,
            );
            self.selected = 0;
        }
        let status = self.nucleo.tick(16);
        if status.changed {
            println!("status update");
            self.update_list();
        }

        CentralPanel::default().show(ctx, |ui| {
            ctx.input(|i| {
                if i.key_pressed(Key::Tab) {
                    if i.modifiers.shift {
                        self.select_up();
                    } else {
                        self.select_down();
                    }
                } else if i.key_pressed(Key::Enter) {
                    self.select();
                } else if i.key_pressed(Key::Escape) {
                    self.close = true;
                }
            });
            ui.add_sized(
                [ui.available_width(), 30.0],
                TextEdit::singleline(&mut self.query)
                    .id_source("search_bar")
                    .vertical_align(Align::Center),
            )
            .request_focus();

            ScrollArea::vertical()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    for (i, item) in self.list.as_slice().iter().enumerate() {
                        let lable = ui.label(item);
                        if i == self.selected {
                            lable.highlight().scroll_to_me(Some(Align::BOTTOM));
                        }
                    }
                });
            ui.with_layout(Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
