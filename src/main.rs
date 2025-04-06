use eframe::egui;
use rand::Rng;
use chrono::prelude::*;
use std::collections::HashSet;

pub struct RandomNumberGeneratorApp {
    lower_bound: i64,
    upper_bound: i64,
    num_to_generate: usize,
    allow_duplicates: bool,
    generated_numbers: Vec<i64>,
    cached_time: String,
    last_update_time: std::time::Instant,
}

impl RandomNumberGeneratorApp {
    pub fn new() -> Self {
        Self {
            lower_bound: 0,
            upper_bound: 1024,
            num_to_generate: 1,
            allow_duplicates: false,
            generated_numbers: Vec::new(),
            cached_time: String::new(),
            last_update_time: std::time::Instant::now(),
        }
    }

    // 修复随机数生成逻辑
    fn generate_numbers(&mut self) {
        let mut rng = rand::rng(); // 使用线程本地随机数生成器
        self.generated_numbers.clear();

        if self.lower_bound > self.upper_bound {
            return;
        }

        if !self.allow_duplicates {
            let range_size = (self.upper_bound - self.lower_bound + 1) as usize;
            if self.num_to_generate > range_size {
                return;
            }

            let mut unique_set = HashSet::new();
            while unique_set.len() < self.num_to_generate {
                let num = rng.random_range(self.lower_bound..=self.upper_bound); 
                unique_set.insert(num);
            }
            self.generated_numbers = unique_set.into_iter().collect();
        } else {
            for _ in 0..self.num_to_generate {
                let num = rng.random_range(self.lower_bound..=self.upper_bound);
                self.generated_numbers.push(num);
            }
        }
    }
}

impl eframe::App for RandomNumberGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Random Number Generator");
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::TOP),
                    |ui| {
                        if ui.button("Dark").clicked() {
                            ctx.set_visuals(egui::Visuals::dark());
                        }
                        if ui.button("Light").clicked() {
                            ctx.set_visuals(egui::Visuals::light());
                        }
                    },
                );
            });

            ui.horizontal(|ui| {
                ui.label("Lower Bound:");
                ui.add(egui::DragValue::new(&mut self.lower_bound));
            });

            ui.horizontal(|ui| {
                ui.label("Upper Bound:");
                ui.add(egui::DragValue::new(&mut self.upper_bound));
            });

            ui.horizontal(|ui| {
                ui.label("Number of Random Numbers to Generate:");
                ui.add(
                    egui::DragValue::new(&mut self.num_to_generate)
                        .range(1..=99999),
                );
            });

            ui.checkbox(&mut self.allow_duplicates, "Allow Duplicates");

            let now = std::time::Instant::now();
            if now.duration_since(self.last_update_time).as_millis() >= 500 {
                let local_time = Local::now();
                self.cached_time = local_time
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();
                self.last_update_time = now;
            }
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.label(&self.cached_time);
                },
            );
            ctx.request_repaint_after(std::time::Duration::from_millis(500));

            ui.separator();
            ui.horizontal(|ui| {
                ui.heading("Generated Numbers");
                if ui.button("Generate").clicked() {
                    self.generate_numbers();
                }
                if ui.button("Clear").clicked() {
                    self.generated_numbers.clear();
                }
            });

            // 恢复逗号分隔的滚动显示
            egui::ScrollArea::vertical() // 支持垂直滚动
                .max_height(160.0) // 设置最大显示高度
                .show(ui, |ui| {
                    ui.label( // 直接显示逗号分隔的字符串
                        self.generated_numbers
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                });

            ui.with_layout(
                egui::Layout::bottom_up(egui::Align::Center),
                |ui| {
                    ui.label("https://github.com/Daihongyi/random-tool-github");
                },
            );

            ui.with_layout(
                egui::Layout::bottom_up(egui::Align::RIGHT),
                |ui| {
                    ui.label("MPL2.0");
                },
            );
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 360.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Random Number Generator",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(RandomNumberGeneratorApp::new()))
        }),
    );
}
