mod random_generator;

use eframe::egui;
use random_generator::RandomGenerator;

struct RandomGeneratorApp {
    gui_version: String,
    generator: RandomGenerator,
    lower_bound: String,
    upper_bound: String,
    num_to_generate: String,
    filename: String,
    error_message: String,
    dark_mode: bool,
    about_open: bool, // 新增字段控制About对话框
    close_requested: bool, // 新增：用于关闭About对话框的请求
}

impl Default for RandomGeneratorApp {
    fn default() -> Self {
        Self {
            gui_version: "v1.0".to_string(),
            generator: RandomGenerator::new(),
            lower_bound: "0".to_owned(),
            upper_bound: "1024".to_owned(),
            num_to_generate: "1".to_owned(),
            filename: "numbers.txt".to_owned(),
            error_message: String::new(),
            dark_mode: false,
            about_open: false, // 默认关闭
            close_requested: false, // 默认无关闭请求
        }
    }
}

impl eframe::App for RandomGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 设置主题
        let mut visuals = egui::Visuals::dark();
        if !self.dark_mode {
            visuals = egui::Visuals::light();
        }
        ctx.set_visuals(visuals);

        // 处理关闭请求（在借用self之前）
        if self.close_requested {
            self.about_open = false;
            self.close_requested = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("✨ Random Number Generator");
                ui.add_space(10.0);

                // 设置面板
                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::same(10.0 as i8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // 边界设置
                            ui.vertical(|ui| {
                                ui.label("Lower Bound:");
                                let lower_input = ui.text_edit_singleline(&mut self.lower_bound);
                                if lower_input.lost_focus() {
                                    if let Ok(num) = self.lower_bound.parse() {
                                        self.generator.set_lower_bound(num);
                                        self.error_message.clear();
                                    } else {
                                        self.error_message = "Invalid lower bound".to_owned();
                                    }
                                }
                            });

                            ui.add_space(10.0);

                            ui.vertical(|ui| {
                                ui.label("Upper Bound:");
                                let upper_input = ui.text_edit_singleline(&mut self.upper_bound);
                                if upper_input.lost_focus() {
                                    if let Ok(num) = self.upper_bound.parse() {
                                        self.generator.set_upper_bound(num);
                                        self.error_message.clear();
                                    } else {
                                        self.error_message = "Invalid upper bound".to_owned();
                                    }
                                }
                            });

                            ui.add_space(10.0);

                            // 数量设置
                            ui.vertical(|ui| {
                                ui.label("Count:");
                                let count_input = ui.text_edit_singleline(&mut self.num_to_generate);
                                if count_input.lost_focus() {
                                    if let Ok(num) = self.num_to_generate.parse() {
                                        self.generator.set_num_to_generate(num);
                                        self.error_message.clear();
                                    } else {
                                        self.error_message = "Invalid count".to_owned();
                                    }
                                }
                            });
                        });

                        ui.add_space(5.0);

                        // 重复选项
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.generator.allow_duplicates, "Allow duplicates");
                            ui.toggle_value(&mut self.dark_mode, "🌙 Dark Mode");
                        });
                    });

                ui.add_space(15.0);

                // 按钮面板
                ui.horizontal(|ui| {
                    if ui.button("🎲 Generate").clicked() {
                        self.generator.generate_numbers();
                        self.error_message.clear();

                        // 验证生成结果
                        let (lower, upper) = self.generator.get_bounds();
                        if lower > upper {
                            self.error_message = "Lower bound > upper bound".to_owned();
                        } else if !self.generator.get_allow_duplicates()
                            && self.generator.get_numbers().len() < self.generator.num_to_generate
                        {
                            self.error_message = "Not enough unique numbers".to_owned();
                        }
                    }

                    if ui.button("🧹 Clear").clicked() {
                        self.generator.clear_numbers();
                        self.error_message.clear();
                    }

                    if ui.button("💾 Save").clicked() {
                        if self.generator.get_numbers().is_empty() {
                            self.error_message = "No numbers to save".to_owned();
                        } else {
                            match self.generator.save_numbers(&self.filename) {
                                Ok(_) => self.error_message = format!("Saved to {}", self.filename),
                                Err(e) => self.error_message = format!("Save error: {}", e),
                            }
                        }
                    }
                });

                // 文件名输入框
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("Filename:");
                    ui.text_edit_singleline(&mut self.filename);
                });

                // 错误信息
                if !self.error_message.is_empty() {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::RED, &self.error_message);
                }

                ui.add_space(20.0);

                // 结果显示
                egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .max_height(300.0)
                    .show(ui, |ui| {
                        egui::Frame::group(ui.style())
                            .fill(ui.visuals().faint_bg_color)
                            .inner_margin(egui::Margin::same(10.0 as i8))
                            .show(ui, |ui| {
                                if self.generator.get_numbers().is_empty() {
                                    ui.centered_and_justified(|ui| {
                                        ui.label("No numbers generated yet");
                                    });
                                } else {
                                    let numbers = self.generator.get_numbers();
                                    let chunk_size = 10;

                                    for chunk in numbers.chunks(chunk_size) {
                                        ui.horizontal(|ui| {
                                            for num in chunk {
                                                ui.monospace(format!("{:>8}", num));
                                            }
                                        });
                                    }

                                    ui.add_space(5.0);
                                    ui.separator();
                                    ui.label(format!(
                                        "Total: {} numbers",
                                        numbers.len()
                                    ));
                                }
                            });
                    });
            });
        });

        // 添加状态栏显示版本信息
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 添加About按钮
                if ui.button("ℹ️ About").clicked() {
                    self.about_open = true;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.label("Random Generator");
                });
            });
        });

        // 添加About对话框
        if self.about_open {
            // 修复：提前复制需要的数据
            let gui_version = self.gui_version.clone();
            let core_version = self.generator.get_core_version().to_string();

            egui::Window::new("About Random Generator")
                .id(egui::Id::new("about_window")) // 添加唯一ID
                .open(&mut self.about_open)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Random Generator");
                        ui.add_space(10.0);

                        ui.label(format!("GUI Version: {}", gui_version));
                        ui.label(format!("Core Version: {}", core_version));

                        ui.add_space(15.0);

                        ui.hyperlink_to("GitHub Repository", "https://github.com/Daihongyi/random-tool-github");

                        ui.add_space(10.0);

                        ui.label("License: MPL-2.0 (Mozilla Public License 2.0)");
                        ui.label("This software is licensed under the terms of the MPL-2.0.");
                        ui.label("Thanks to the open-source community");
                        ui.label("Develop on RustRover");
                        ui.add_space(15.0);

                        if ui.button("Close").clicked() {
                            // 设置关闭请求标志，而不是直接修改about_open
                            self.close_requested = true;
                        }
                    });
                });
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Random Generator",
        options,
        Box::new(|_cc| Ok(Box::new(RandomGeneratorApp::default()))),
    )
}