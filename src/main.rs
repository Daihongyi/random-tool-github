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
    about_open: bool,
    close_requested: bool,
}

impl Default for RandomGeneratorApp {
    fn default() -> Self {
        Self {
            gui_version: "v1.1".to_string(),
            generator: RandomGenerator::new(),
            lower_bound: "0".to_owned(),
            upper_bound: "1024".to_owned(),
            num_to_generate: "1".to_owned(),
            filename: "numbers.txt".to_owned(),
            error_message: String::new(),
            dark_mode:false, // Default to dark mode for cool factor
            about_open: false,
            close_requested: false,
        }
    }
}

impl eframe::App for RandomGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Custom dark theme with cool colors
        let mut visuals = if self.dark_mode {
            let mut v = egui::Visuals::dark();
            v.override_text_color = Some(egui::Color32::from_rgb(220, 220, 220));
            v.window_fill = egui::Color32::from_rgb(25, 25, 35);
            v.panel_fill = egui::Color32::from_rgb(30, 30, 40);
            v.faint_bg_color = egui::Color32::from_rgb(40, 40, 50);
            v.extreme_bg_color = egui::Color32::from_rgb(20, 20, 30);
            v.widgets.active.bg_fill = egui::Color32::from_rgb(80, 120, 255);
            v.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 100, 235);
            v.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 45, 55);
            v
        } else {
            egui::Visuals::light()
        };
        ctx.set_visuals(visuals);

        // Handle close request
        if self.close_requested {
            self.about_open = false;
            self.close_requested = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Compact header with gradient-like effect
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.heading("🎲 Random Generator");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Compact theme toggle
                        let theme_icon = if self.dark_mode { "☀️" } else { "🌙" };
                        if ui.small_button(theme_icon).clicked() {
                            self.dark_mode = !self.dark_mode;
                        }
                    });
                });
                ui.add_space(12.0);

                // Compact input section with rounded corners
                egui::Frame::group(ui.style())
                    .fill(ui.visuals().faint_bg_color)
                    .inner_margin(egui::Margin::same(12.0 as i8))
                    .corner_radius(egui::CornerRadius::same(8.0 as u8))
                    .show(ui, |ui| {
                        // First row: bounds and count
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label("From");
                                ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut self.lower_bound));
                            });

                            ui.add_space(8.0);
                            ui.label("→");
                            ui.add_space(8.0);

                            ui.vertical(|ui| {
                                ui.label("To");
                                ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut self.upper_bound));
                            });

                            ui.add_space(12.0);
                            ui.separator();
                            ui.add_space(12.0);

                            ui.vertical(|ui| {
                                ui.label("Count");
                                ui.add_sized([50.0, 20.0], egui::TextEdit::singleline(&mut self.num_to_generate));
                            });
                        });

                        ui.add_space(8.0);

                        // Options row
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.generator.allow_duplicates, "Allow duplicates");
                        });
                    });

                ui.add_space(10.0);

                // Compact button row
                ui.horizontal(|ui| {
                    let button_size = egui::vec2(80.0, 28.0);

                    if ui.add_sized(button_size, egui::Button::new("🎲 Generate").corner_radius(egui::CornerRadius::same(4.0 as u8))).clicked() {
                        // Update generator with current values
                        if let Ok(lower) = self.lower_bound.parse() {
                            self.generator.set_lower_bound(lower);
                        }
                        if let Ok(upper) = self.upper_bound.parse() {
                            self.generator.set_upper_bound(upper);
                        }
                        if let Ok(count) = self.num_to_generate.parse() {
                            self.generator.set_num_to_generate(count);
                        }

                        self.generator.generate_numbers();
                        self.error_message.clear();

                        // Validate results
                        let (lower, upper) = self.generator.get_bounds();
                        if lower > upper {
                            self.error_message = "Lower bound > upper bound".to_owned();
                        } else if !self.generator.get_allow_duplicates()
                            && self.generator.get_numbers().len() < self.generator.num_to_generate
                        {
                            self.error_message = "Not enough unique numbers".to_owned();
                        }
                    }

                    if ui.add_sized(button_size, egui::Button::new("🧹 Clear").corner_radius(egui::CornerRadius::same(4.0 as u8))).clicked() {
                        self.generator.clear_numbers();
                        self.error_message.clear();
                    }

                    if ui.add_sized(button_size, egui::Button::new("💾 Save").corner_radius(egui::CornerRadius::same(4.0 as u8))).clicked() {
                        if self.generator.get_numbers().is_empty() {
                            self.error_message = "No numbers to save".to_owned();
                        } else {
                            match self.generator.save_numbers(&self.filename) {
                                Ok(_) => self.error_message = format!("✅ Saved to {}", self.filename),
                                Err(e) => self.error_message = format!("❌ Save error: {}", e),
                            }
                        }
                    }
                });

                // Compact filename input
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label("File:");
                    ui.add_sized([120.0, 20.0], egui::TextEdit::singleline(&mut self.filename));
                });

                // Error message with better styling
                if !self.error_message.is_empty() {
                    ui.add_space(6.0);
                    let color = if self.error_message.starts_with("✅") {
                        egui::Color32::from_rgb(100, 200, 100)
                    } else {
                        egui::Color32::from_rgb(255, 100, 100)
                    };
                    ui.colored_label(color, &self.error_message);
                }

                ui.add_space(12.0);

                // Compact results display
                egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .max_height(180.0)
                    .show(ui, |ui| {
                        egui::Frame::group(ui.style())
                            .fill(ui.visuals().extreme_bg_color)
                            .inner_margin(egui::Margin::same(8.0 as i8))
                            .corner_radius(egui::Rounding::same(6.0 as u8))
                            .show(ui, |ui| {
                                if self.generator.get_numbers().is_empty() {
                                    ui.centered_and_justified(|ui| {
                                        ui.label("🎯 Click Generate to start");
                                    });
                                } else {
                                    let numbers = self.generator.get_numbers();
                                    let chunk_size = 12; // More numbers per row

                                    for chunk in numbers.chunks(chunk_size) {
                                        ui.horizontal(|ui| {
                                            for num in chunk {
                                                ui.monospace(format!("{:>6}", num));
                                            }
                                        });
                                    }

                                    ui.add_space(4.0);
                                    ui.separator();
                                    ui.small(format!("📊 Total: {} numbers", numbers.len()));
                                }
                            });
                    });
            });
        });

        // Compact status bar
        egui::TopBottomPanel::bottom("status_bar")
            .exact_height(24.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.small_button("ℹ️").clicked() {
                        self.about_open = true;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.small("Random Generator");
                    });
                });
            });

        // Compact About dialog
        if self.about_open {
            let gui_version = self.gui_version.clone();
            let core_version = self.generator.get_core_version().to_string();

            egui::Window::new("About")
                .id(egui::Id::new("about_window"))
                .open(&mut self.about_open)
                .resizable(false)
                .collapsible(false)
                .default_size([280.0, 200.0])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("🎲 Random Generator");
                        ui.add_space(8.0);

                        ui.small(format!("GUI: {}", gui_version));
                        ui.small(format!("Core: {}", core_version));

                        ui.add_space(10.0);

                        ui.hyperlink_to("🔗 GitHub", "https://github.com/Daihongyi/random-tool-github");

                        ui.add_space(8.0);

                        ui.small("📄 License: MPL-2.0");
                        ui.small("🦀 Built with Rust");
                        ui.small("💎 Powered by egui");

                        ui.add_space(12.0);

                        if ui.button("Close").clicked() {
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
            .with_inner_size([480.0, 420.0]) // Much smaller window
            .with_min_inner_size([400.0, 350.0])
            .with_max_inner_size([600.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Random Generator",
        options,
        Box::new(|_cc| Ok(Box::new(RandomGeneratorApp::default()))),
    )
}