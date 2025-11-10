use std::cmp;

use egui::{Align, CentralPanel, Key, Layout, TextEdit};
use nucleo::Nucleo;

use crate::desktop::Desktop;

pub struct App {
    query: String,
    last_query: String,
    selected: usize,
    nucleo: Nucleo<Desktop>,
}

impl App {
    pub fn new(nucleo: Nucleo<Desktop>) -> Self {
        Self {
            query: String::from(""),
            last_query: String::from(""),
            selected: 0,
            nucleo,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
        self.nucleo.tick(16);
        let snapshot = self.nucleo.snapshot();

        CentralPanel::default().show(ctx, |ui| {
            ctx.input(|i| {
                if i.key_pressed(Key::Tab) {
                    if i.modifiers.shift {
                        if self.selected == 0 {
                            self.selected = snapshot.matched_item_count() as usize - 1;
                        } else {
                            self.selected -= 1;
                        }
                    } else {
                        if self.selected + 1 >= snapshot.matched_item_count() as usize {
                            self.selected = 0;
                        } else {
                            self.selected += 1;
                        }
                    }
                }
            });
            ui.add_sized(
                [ui.available_width(), 30.0],
                TextEdit::singleline(&mut self.query)
                    .id_source("search_bar")
                    .vertical_align(Align::Center),
            )
            .request_focus();

            ui.vertical(|ui| {
                for (i, item) in snapshot.matched_items(..).enumerate() {
                    let lable = ui.label(&item.data.name);
                    if i == self.selected {
                        lable.highlight();
                    }
                }
            });

            ui.with_layout(Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
