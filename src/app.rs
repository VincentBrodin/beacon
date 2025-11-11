use std::process::Command;

use egui::{Align, CentralPanel, Key, Layout, TextEdit};
use nucleo::{Item, Nucleo};

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

        CentralPanel::default().show(ctx, |ui| {
            let snapshot = self.nucleo.snapshot();
            let matched_count = snapshot.matched_item_count();
            ctx.input(|i| {
                if i.key_pressed(Key::Tab) {
                    if i.modifiers.shift {
                        if self.selected == 0 {
                            self.selected = matched_count as usize - 1;
                        } else {
                            self.selected -= 1;
                        }
                    } else {
                        if self.selected + 1 >= matched_count as usize {
                            self.selected = 0;
                        } else {
                            self.selected += 1;
                        }
                    }
                } else if i.key_pressed(Key::Enter) {
                    let selected = self.selected as u32;
                    let items: Vec<_> = self
                        .nucleo
                        .snapshot()
                        .matched_items(selected..selected + 1)
                        .collect();
                    match items.first() {
                        Some(item) => {
                            println!("User selected {}", item.data.name);
                            match &item.data.entry_type {
                                crate::desktop::Type::Application(exec) => match exec {
                                    Some(exec) => {
                                        println!("Should execute {}", exec);
                                        // let result = Command::new(exec).output();
                                        // match result {
                                        //     Ok(output) => println!(
                                        //         "{} succseded with code {}",
                                        //         exec, output.status
                                        //     ),
                                        //     Err(err) => println!("{} failed: {}", exec, err),
                                        // }
                                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                    }
                                    None => println!("App does not have an executable link"),
                                },
                                crate::desktop::Type::Link(_) => todo!(),
                                crate::desktop::Type::Directory => todo!(),
                            }
                        }
                        None => println!("Miss (Should not happen)"),
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
