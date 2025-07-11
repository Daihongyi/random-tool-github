mod random_generator;

use iced::widget::{
    button, checkbox, column, container, row, scrollable, text, text_input, Space
};
use iced::{
    alignment, Element, Length, Theme, Color, Background, Border, Shadow, Vector, Task
};
use random_generator::RandomGenerator;

// 定义 Emoji 字体常量
const EMOJI_FONT: iced::Font = iced::Font::with_name("Segoe UI Emoji");

#[derive(Debug, Clone)]
pub enum Message {
    LowerBoundChanged(String),
    UpperBoundChanged(String),
    NumToGenerateChanged(String),
    FilenameChanged(String),
    AllowDuplicatesToggled(bool),
    Generate,
    Clear,
    Save,
    ToggleTheme,
    ShowAbout,
    CloseAbout,
}

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
    theme: Theme,
}

impl Default for RandomGeneratorApp {
    fn default() -> Self {
        let generator = RandomGenerator::new();
        let config = generator.get_config();
        // 提取配置值并结束借用
        let lower_bound = config.lower_bound.to_string();
        let upper_bound = config.upper_bound.to_string();
        let num_to_generate = config.num_to_generate.to_string();

        Self {
            gui_version: "v1.2".to_string(),
            generator,
            lower_bound,
            upper_bound,
            num_to_generate,
            filename: "numbers.txt".to_owned(),
            error_message: String::new(),
            dark_mode: false,
            about_open: false,
            theme: Theme::Light,
        }
    }
}

impl RandomGeneratorApp {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn title(&self) -> String {
        String::from("Random Generator")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LowerBoundChanged(value) => {
                self.lower_bound = value;
            }
            Message::UpperBoundChanged(value) => {
                self.upper_bound = value;
            }
            Message::NumToGenerateChanged(value) => {
                self.num_to_generate = value;
            }
            Message::FilenameChanged(value) => {
                self.filename = value;
            }
            Message::AllowDuplicatesToggled(value) => {
                if let Err(e) = self.generator.set_allow_duplicates(value) {
                    self.error_message = e.to_string();
                }
            }
            Message::Generate => {
                // 清除之前的错误信息
                self.error_message.clear();

                // 解析并设置下界
                if let Ok(lower) = self.lower_bound.parse() {
                    if let Err(e) = self.generator.set_lower_bound(lower) {
                        self.error_message = e.to_string();
                        return Task::none();
                    }
                } else {
                    self.error_message = "下界必须是整数".to_string();
                    return Task::none();
                }

                // 解析并设置上界
                if let Ok(upper) = self.upper_bound.parse() {
                    if let Err(e) = self.generator.set_upper_bound(upper) {
                        self.error_message = e.to_string();
                        return Task::none();
                    }
                } else {
                    self.error_message = "上界必须是整数".to_string();
                    return Task::none();
                }

                // 解析并设置生成数量
                if let Ok(count) = self.num_to_generate.parse() {
                    if let Err(e) = self.generator.set_num_to_generate(count) {
                        self.error_message = e.to_string();
                        return Task::none();
                    }
                } else {
                    self.error_message = "生成数量必须是整数".to_string();
                    return Task::none();
                }

                // 生成随机数
                if let Err(e) = self.generator.generate_numbers() {
                    self.error_message = e.to_string();
                }
            }
            Message::Clear => {
                self.generator.clear_numbers();
                self.error_message.clear();
            }
            Message::Save => {
                if self.generator.get_numbers().is_empty() {
                    self.error_message = "没有数字可保存".to_owned();
                } else {
                    match self.generator.save_numbers(&self.filename) {
                        Ok(_) => self.error_message = format!("✅ 保存到 {}", self.filename),
                        Err(e) => self.error_message = format!("❌ 保存错误: {}", e),
                    }
                }
            }
            Message::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                self.theme = if self.dark_mode {
                    Theme::Dark
                } else {
                    Theme::Light
                };
            }
            Message::ShowAbout => {
                self.about_open = true;
            }
            Message::CloseAbout => {
                self.about_open = false;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            text("🎲 Random Generator")
                .font(EMOJI_FONT)  // 添加 Emoji 字体
                .size(16)
                .color(if self.dark_mode {
                    Color::from_rgb(0.9, 0.9, 0.9)
                } else {
                    Color::BLACK
                }),
            Space::with_width(Length::Fill),
            button(text(if self.dark_mode { "☀️" } else { "🌙" })
                .font(EMOJI_FONT)  // 添加 Emoji 字体
                .size(11))
                .on_press(Message::ToggleTheme)
                .style(move |_theme: &Theme, status| {
                    let is_pressed = status == button::Status::Pressed;
                    button::Style {
                        background: Some(Background::Color(
                            if is_pressed {
                                if self.dark_mode {
                                    Color::from_rgb(0.2, 0.2, 0.25)
                                } else {
                                    Color::from_rgb(0.8, 0.8, 0.85)
                                }
                            } else if self.dark_mode {
                                Color::from_rgb(0.3, 0.3, 0.35)
                            } else {
                                Color::from_rgb(0.9, 0.9, 0.9)
                            }
                        )),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 12.0.into(),
                        },
                        text_color: if self.dark_mode {
                            Color::from_rgb(0.9, 0.9, 0.9)
                        } else {
                            Color::BLACK
                        },
                        shadow: Shadow {
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                            offset: Vector::new(0.0, if is_pressed { 1.0 } else { 2.0 }),
                            blur_radius: if is_pressed { 2.0 } else { 4.0 },
                        },
                        ..Default::default()
                    }
                })
        ]
            .spacing(4)
            .align_y(alignment::Vertical::Center);

        let input_section = container(
            column![
                // Input row - more compact
                row![
                    // From input
                    column![
                        text("From").size(11),
                        text_input("", &self.lower_bound)
                            .on_input(Message::LowerBoundChanged)
                            .width(Length::Fixed(50.0))
                            .size(11)
                            .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode))
                    ]
                    .spacing(1),

                    Space::with_width(Length::Fixed(4.0)),
                    text("→").size(12),
                    Space::with_width(Length::Fixed(4.0)),

                    // To input
                    column![
                        text("To").size(11),
                        text_input("", &self.upper_bound)
                            .on_input(Message::UpperBoundChanged)
                            .width(Length::Fixed(50.0))
                            .size(11)
                            .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode))
                    ]
                    .spacing(1),

                    Space::with_width(Length::Fixed(8.0)),

                    // Count input
                    column![
                        text("Count").size(11),
                        text_input("", &self.num_to_generate)
                            .on_input(Message::NumToGenerateChanged)
                            .width(Length::Fixed(40.0))
                            .size(11)
                            .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode))
                    ]
                    .spacing(1)
                ]
                .spacing(4)
                .align_y(alignment::Vertical::Bottom),

                Space::with_height(Length::Fixed(4.0)),

                // Checkbox - more compact
                checkbox("Allow duplicates", self.generator.get_allow_duplicates())
                    .on_toggle(Message::AllowDuplicatesToggled)
                    .size(11)
                    .text_size(10)
                    .style(move |_theme: &Theme, _status| {
                        checkbox::Style {
                            background: Background::Color(
                                if self.dark_mode {
                                    Color::from_rgb(0.25, 0.25, 0.3)
                                } else {
                                    Color::WHITE
                                }
                            ),
                            icon_color: if self.dark_mode {
                                Color::from_rgb(0.5, 0.8, 0.5)
                            } else {
                                Color::from_rgb(0.2, 0.6, 0.2)
                            },
                            border: Border {
                                color: if self.dark_mode {
                                    Color::from_rgb(0.4, 0.4, 0.45)
                                } else {
                                    Color::from_rgb(0.8, 0.8, 0.8)
                                },
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            text_color: Some(if self.dark_mode {
                                Color::from_rgb(0.9, 0.9, 0.9)
                            } else {
                                Color::BLACK
                            }),
                        }
                    })
            ]
                .spacing(3)
                .padding(6)
        )
            .style(move |_theme: &Theme| {
                iced::widget::container::Style {
                    background: Some(Background::Color(
                        if self.dark_mode {
                            Color::from_rgb(0.2, 0.2, 0.25)
                        } else {
                            Color::from_rgb(0.96, 0.96, 0.96)
                        }
                    )),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 10.0.into(),
                    },
                    shadow: Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: Vector::new(0.0, 2.0),
                        blur_radius: 4.0,
                    },
                    ..Default::default()
                }
            });

        let button_row = row![
            button(text("Generate").size(11))
                .on_press(Message::Generate)
                .width(Length::Fixed(70.0))
                .style(move |_theme: &Theme, status| {
                    let is_pressed = status == button::Status::Pressed;
                    button::Style {
                        background: Some(Background::Color(
                            if is_pressed {
                                if self.dark_mode {
                                    Color::from_rgb(0.2, 0.4, 0.7)
                                } else {
                                    Color::from_rgb(0.1, 0.5, 0.8)
                                }
                            } else if self.dark_mode {
                                Color::from_rgb(0.3, 0.5, 0.8)
                            } else {
                                Color::from_rgb(0.2, 0.6, 0.9)
                            }
                        )),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 8.0.into(),
                        },
                        text_color: Color::WHITE,
                        shadow: Shadow {
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                            offset: Vector::new(0.0, if is_pressed { 1.0 } else { 2.0 }),
                            blur_radius: if is_pressed { 2.0 } else { 4.0 },
                        },
                        ..Default::default()
                    }
                }),

            button(text("Clear").size(11))
                .on_press(Message::Clear)
                .width(Length::Fixed(55.0))
                .style(move |_theme: &Theme, status| {
                    let is_pressed = status == button::Status::Pressed;
                    button::Style {
                        background: Some(Background::Color(
                            if is_pressed {
                                if self.dark_mode {
                                    Color::from_rgb(0.5, 0.2, 0.2)
                                } else {
                                    Color::from_rgb(0.8, 0.3, 0.3)
                                }
                            } else if self.dark_mode {
                                Color::from_rgb(0.6, 0.3, 0.3)
                            } else {
                                Color::from_rgb(0.9, 0.4, 0.4)
                            }
                        )),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 8.0.into(),
                        },
                        text_color: Color::WHITE,
                        shadow: Shadow {
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                            offset: Vector::new(0.0, if is_pressed { 1.0 } else { 2.0 }),
                            blur_radius: if is_pressed { 2.0 } else { 4.0 },
                        },
                        ..Default::default()
                    }
                }),

            button(text("Save").size(11))
                .on_press(Message::Save)
                .width(Length::Fixed(55.0))
                .style(move |_theme: &Theme, status| {
                    let is_pressed = status == button::Status::Pressed;
                    button::Style {
                        background: Some(Background::Color(
                            if is_pressed {
                                if self.dark_mode {
                                    Color::from_rgb(0.2, 0.5, 0.2)
                                } else {
                                    Color::from_rgb(0.3, 0.7, 0.3)
                                }
                            } else if self.dark_mode {
                                Color::from_rgb(0.3, 0.6, 0.3)
                            } else {
                                Color::from_rgb(0.4, 0.8, 0.4)
                            }
                        )),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 8.0.into(),
                        },
                        text_color: Color::WHITE,
                        shadow: Shadow {
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                            offset: Vector::new(0.0, if is_pressed { 1.0 } else { 2.0 }),
                            blur_radius: if is_pressed { 2.0 } else { 4.0 },
                        },
                        ..Default::default()
                    }
                })
        ]
            .spacing(4);

        let filename_input = container(
            row![
                text("File:").size(12),
                text_input("", &self.filename)
                    .on_input(Message::FilenameChanged)
                    .width(Length::Fixed(100.0))
                    .size(10)
                    .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode))
            ]
                .spacing(4)
                .align_y(alignment::Vertical::Center)
        )
            .padding(2);

        let error_display = if !self.error_message.is_empty() {
            container(
                text(&self.error_message)
                    .size(10)
                    .style(move |_theme: &Theme| {
                        iced::widget::text::Style {
                            color: Some(if self.error_message.starts_with("✅") {
                                Color::from_rgb(0.4, 0.8, 0.4)
                            } else {
                                Color::from_rgb(1.0, 0.4, 0.4)
                            }),
                        }
                    })
            )
                .padding(2)
                .style(move |_theme: &Theme| {
                    iced::widget::container::Style {
                        background: Some(Background::Color(
                            if self.dark_mode {
                                Color::from_rgba(0.2, 0.2, 0.25, 0.8)
                            } else {
                                Color::from_rgba(0.95, 0.95, 0.95, 0.8)
                            }
                        )),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 6.0.into(),
                        },
                        ..Default::default()
                    }
                })
        } else {
            container(Space::with_height(Length::Fixed(0.0)))
        };

        let results_display = if self.generator.get_numbers().is_empty() {
            container(
                text("🎯 Click Generate to start")
                    .font(EMOJI_FONT)  // 添加 Emoji 字体
                    .size(12)
                    .style(move |_theme: &Theme| {
                        iced::widget::text::Style {
                            color: Some(if self.dark_mode {
                                Color::from_rgb(0.6, 0.6, 0.6)
                            } else {
                                Color::from_rgb(0.5, 0.5, 0.5)
                            }),
                        }
                    })
            )
                .center_x(Length::Fill)
                .center_y(Length::Fixed(80.0))
                .width(Length::Fill)
                .height(Length::Fixed(80.0))
                .style(move |_theme: &Theme| {
                    iced::widget::container::Style {
                        background: Some(Background::Color(
                            if self.dark_mode {
                                Color::from_rgb(0.15, 0.15, 0.20)
                            } else {
                                Color::from_rgb(0.98, 0.98, 0.98)
                            }
                        )),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 8.0.into(),
                        },
                        ..Default::default()
                    }
                })
        } else {
            let numbers = self.generator.get_numbers();
            let chunk_size = 8; // Fewer numbers per row for compact display

            let mut rows = Vec::new();
            for chunk in numbers.chunks(chunk_size) {
                let number_row = row(
                    chunk.iter().map(|num| {
                        container(
                            text(format!("{}", num))
                                .size(10)
                                .font(iced::Font::MONOSPACE)
                        )
                            .padding(2)
                            .style(move |_theme: &Theme| {
                                iced::widget::container::Style {
                                    background: Some(Background::Color(
                                        if self.dark_mode {
                                            Color::from_rgb(0.25, 0.25, 0.3)
                                        } else {
                                            Color::from_rgb(0.92, 0.92, 0.92)
                                        }
                                    )),
                                    border: Border {
                                        color: Color::TRANSPARENT,
                                        width: 0.0,
                                        radius: 4.0.into(),
                                    },
                                    ..Default::default()
                                }
                            })
                            .into()
                    }).collect::<Vec<_>>()
                )
                    .spacing(2);
                rows.push(number_row.into());
            }

            // Add total count
            rows.push(Space::with_height(Length::Fixed(4.0)).into());
            rows.push(
                container(
                    text(format!("📊 Total: {}", numbers.len()))
                        .font(EMOJI_FONT)  // 添加 Emoji 字体
                        .size(10)
                        .style(move |_theme: &Theme| {
                            iced::widget::text::Style {
                                color: Some(if self.dark_mode {
                                    Color::from_rgb(0.6, 0.6, 0.6)
                                } else {
                                    Color::from_rgb(0.5, 0.5, 0.5)
                                }),
                            }
                        })
                )
                    .center_x(Length::Fill)
                    .into()
            );

            container(
                scrollable(
                    column(rows)
                        .spacing(2)
                        .padding(4)
                )
                    .height(Length::Fixed(90.0))
            )
                .style(move |_theme: &Theme| {
                    iced::widget::container::Style {
                        background: Some(Background::Color(
                            if self.dark_mode {
                                Color::from_rgb(0.15, 0.15, 0.20)
                            } else {
                                Color::from_rgb(0.98, 0.98, 0.98)
                            }
                        )),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 8.0.into(),
                        },
                        ..Default::default()
                    }
                })
        };

        let status_bar = row![
            button(text("ℹ️")
                .font(EMOJI_FONT)  // 添加 Emoji 字体
                .size(12))
                .on_press(Message::ShowAbout)
                .style(move |_theme: &Theme, status| {
                    let is_pressed = status == button::Status::Pressed;
                    button::Style {
                        background: Some(Background::Color(
                            if is_pressed {
                                if self.dark_mode {
                                    Color::from_rgb(0.2, 0.2, 0.25)
                                } else {
                                    Color::from_rgb(0.9, 0.9, 0.9)
                                }
                            } else {
                                Color::TRANSPARENT
                            }
                        )),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 8.0.into(),
                        },
                        text_color: if self.dark_mode {
                            Color::from_rgb(0.7, 0.7, 0.7)
                        } else {
                            Color::from_rgb(0.5, 0.5, 0.5)
                        },
                        ..Default::default()
                    }
                }),
            Space::with_width(Length::Fill),
            text("Random Generator")
                .size(10)
                .color(if self.dark_mode {
                    Color::from_rgb(0.6, 0.6, 0.6)
                } else {
                    Color::from_rgb(0.5, 0.5, 0.5)
                })
        ]
            .spacing(4)
            .align_y(alignment::Vertical::Center);

        let main_content = column![
            header,
            Space::with_height(Length::Fixed(8.0)),
            input_section,
            Space::with_height(Length::Fixed(8.0)),
            button_row,
            Space::with_height(Length::Fixed(6.0)),
            filename_input,
            Space::with_height(Length::Fixed(4.0)),
            error_display,
            Space::with_height(Length::Fixed(8.0)),
            results_display,
            Space::with_height(Length::Fill),
            status_bar
        ]
            .spacing(0)
            .padding(12);

        if self.about_open {
            let about_content = container(
                column![
                    text("🎲 Random Generator")
                        .font(EMOJI_FONT)  // 添加 Emoji 字体
                        .size(18)
                        .color(if self.dark_mode { Color::from_rgb(0.9, 0.9, 0.9) } else { Color::BLACK }),
                    Space::with_height(Length::Fixed(8.0)),
                    text(format!("GUI: {}", self.gui_version))
                        .size(12),
                    text(format!("Core: {}", self.generator.get_core_version()))
                        .size(12),
                    Space::with_height(Length::Fixed(12.0)),
                    text("🔗 GitHub: https://github.com/Daihongyi/random-tool-github")
                        .font(EMOJI_FONT)  // 添加 Emoji 字体
                        .size(10),
                    Space::with_height(Length::Fixed(8.0)),
                    text("📄 License: MPL-2.0")
                        .font(EMOJI_FONT)  // 添加 Emoji 字体
                        .size(10),
                    text("🦀 Built with Rust")
                        .font(EMOJI_FONT)  // 添加 Emoji 字体
                        .size(10),
                    text("❄️ Powered by Iced")
                        .font(EMOJI_FONT)  // 添加 Emoji 字体
                        .size(10),
                    Space::with_height(Length::Fixed(16.0)),
                    button(text("Close").size(12))
                        .on_press(Message::CloseAbout)
                        .width(Length::Fixed(70.0))
                        .style(move |_theme: &Theme, status| {
                            let is_pressed = status == button::Status::Pressed;
                            button::Style {
                                background: Some(Background::Color(
                                    if is_pressed {
                                        if self.dark_mode {
                                            Color::from_rgb(0.2, 0.2, 0.25)
                                        } else {
                                            Color::from_rgb(0.1, 0.5, 0.8)
                                        }
                                    } else if self.dark_mode {
                                        Color::from_rgb(0.3, 0.3, 0.35)
                                    } else {
                                        Color::from_rgb(0.2, 0.6, 0.9)
                                    }
                                )),
                                border: Border {
                                    color: Color::TRANSPARENT,
                                    width: 0.0,
                                    radius: 8.0.into(),
                                },
                                text_color: Color::WHITE,
                                shadow: Shadow {
                                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                                    offset: Vector::new(0.0, if is_pressed { 1.0 } else { 2.0 }),
                                    blur_radius: if is_pressed { 2.0 } else { 4.0 },
                                },
                                ..Default::default()
                            }
                        })
                ]
                    .spacing(3)
                    .align_x(alignment::Horizontal::Center)
                    .padding(20)
            )
                .center_x(Length::Fixed(280.0))
                .center_y(Length::Fixed(240.0))
                .width(Length::Fixed(280.0))
                .height(Length::Fixed(240.0))
                .style(move |_theme: &Theme| {
                    iced::widget::container::Style {
                        background: Some(Background::Color(
                            if self.dark_mode {
                                Color::from_rgb(0.2, 0.2, 0.25)
                            } else {
                                Color::WHITE
                            }
                        )),
                        border: Border {
                            color: if self.dark_mode {
                                Color::from_rgb(0.4, 0.4, 0.4)
                            } else {
                                Color::from_rgb(0.8, 0.8, 0.8)
                            },
                            width: 1.0,
                            radius: 16.0.into(),
                        },
                        shadow: Shadow {
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                            offset: Vector::new(0.0, 4.0),
                            blur_radius: 20.0,
                        },
                        ..Default::default()
                    }
                });

            container(
                container(about_content)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
                .style(move |_theme: &Theme| {
                    iced::widget::container::Style {
                        background: Some(Background::Color(
                            Color::from_rgba(0.0, 0.0, 0.0, 0.5)
                        )),
                        ..Default::default()
                    }
                })
                .width(Length::Fill)
                .height(Length::Fill).into()
        } else {
            container(main_content)
                .width(Length::Fill)
                .height(Length::Fill).into()
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

// 定义获取文本输入框样式的函数
fn get_text_input_style(dark_mode: bool) -> text_input::Style {
    text_input::Style {
        background: Background::Color(
            if dark_mode {
                Color::from_rgb(0.25, 0.25, 0.3)
            } else {
                Color::WHITE
            }
        ),
        border: Border {
            color: if dark_mode {
                Color::from_rgb(0.4, 0.4, 0.45)
            } else {
                Color::from_rgb(0.8, 0.8, 0.8)
            },
            width: 1.0,
            radius: 6.0.into(),
        },
        icon: Color::TRANSPARENT,
        placeholder: if dark_mode {
            Color::from_rgb(0.6, 0.6, 0.6)
        } else {
            Color::from_rgb(0.4, 0.4, 0.4)
        },
        value: if dark_mode {
            Color::from_rgb(0.9, 0.9, 0.9)
        } else {
            Color::BLACK
        },
        selection: Color::from_rgb(0.5, 0.7, 1.0),
    }
}

fn main() -> iced::Result {
    iced::application(
        RandomGeneratorApp::title,
        RandomGeneratorApp::update,
        RandomGeneratorApp::view
    )
        .theme(RandomGeneratorApp::theme)
        .window(iced::window::Settings {
            size: iced::Size::new(400.0, 360.0),
            position: Default::default(),
            min_size: Some(iced::Size::new(280.0, 360.0)),
            max_size: Some(iced::Size::new(400.0, 600.0)),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: iced::window::Level::Normal,
            icon: None,
            platform_specific: Default::default(),
            exit_on_close_request: true,
        })
        .run_with(RandomGeneratorApp::new)
}