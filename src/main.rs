mod random_generator;

use iced::widget::{
    button, checkbox, column, container, horizontal_rule, pick_list, row, scrollable, text, text_input, Space
};
use iced::{
    alignment, Element, Length, Theme, Color, Background, Border, Shadow, Vector, Task
};
use random_generator::{RandomGenerator, GeneratorMode};
use std::fmt;

// Implement Display trait for GeneratorMode
impl fmt::Display for GeneratorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeneratorMode::Range => write!(f, "Range"),
            GeneratorMode::CustomList => write!(f, "Custom List"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LowerBoundChanged(String),
    UpperBoundChanged(String),
    NumToGenerateChanged(String),
    FilenameChanged(String),
    AllowDuplicatesToggled(bool),
    ModeChanged(GeneratorMode),
    CustomListChanged(String),
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
    mode: GeneratorMode,
    custom_list_input: String,
}

impl Default for RandomGeneratorApp {
    fn default() -> Self {
        let generator = RandomGenerator::new();
        let config = generator.get_config();
        // Extract config values and end borrow
        let lower_bound = config.lower_bound.to_string();
        let upper_bound = config.upper_bound.to_string();
        let num_to_generate = config.num_to_generate.to_string();
        let mode = config.mode.clone();
        let custom_list_input = config.custom_list_input.clone();

        Self {
            gui_version: "v1.3".to_string(),
            generator,
            lower_bound,
            upper_bound,
            num_to_generate,
            filename: "numbers.txt".to_owned(),
            error_message: String::new(),
            dark_mode: false,
            about_open: false,
            theme: Theme::Light,
            mode,
            custom_list_input,
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
            Message::ModeChanged(mode) => {
                self.mode = mode.clone();
                if let Err(e) = self.generator.set_mode(mode) {
                    self.error_message = e.to_string();
                }
            }
            Message::CustomListChanged(value) => {
                self.custom_list_input = value.clone();
                if let Err(e) = self.generator.set_custom_list_input(value) {
                    self.error_message = e.to_string();
                }
            }
            Message::Generate => {
                // Clear previous error message
                self.error_message.clear();

                // If range mode, parse and set bounds
                if self.mode == GeneratorMode::Range {
                    // Parse and set lower bound
                    if let Ok(lower) = self.lower_bound.parse() {
                        if let Err(e) = self.generator.set_lower_bound(lower) {
                            self.error_message = e.to_string();
                            return Task::none();
                        }
                    } else {
                        self.error_message = "Lower bound must be an integer".to_string();
                        return Task::none();
                    }

                    // Parse and set upper bound
                    if let Ok(upper) = self.upper_bound.parse() {
                        if let Err(e) = self.generator.set_upper_bound(upper) {
                            self.error_message = e.to_string();
                            return Task::none();
                        }
                    } else {
                        self.error_message = "Upper bound must be an integer".to_string();
                        return Task::none();
                    }
                }

                // Parse and set generation count
                if let Ok(count) = self.num_to_generate.parse() {
                    if let Err(e) = self.generator.set_num_to_generate(count) {
                        self.error_message = e.to_string();
                        return Task::none();
                    }
                } else {
                    self.error_message = "Count must be an integer".to_string();
                    return Task::none();
                }

                // Generate random numbers
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
                    self.error_message = "No numbers to save".to_owned();
                } else {
                    match self.generator.save_numbers(&self.filename) {
                        Ok(_) => self.error_message = format!("Saved to {}", self.filename),
                        Err(e) => self.error_message = format!("Save error: {}", e),
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
            text("Random Generator")
                .size(18)
                .color(if self.dark_mode {
                    Color::from_rgb(0.9, 0.9, 0.9)
                } else {
                    Color::BLACK
                }),
            Space::with_width(Length::Fill),
            button(text(if self.dark_mode { "Light" } else { "Dark" })
                .size(14))
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

        // Mode picker
        let mode_picker = container(
            row![
                text("Mode:").size(14),
                pick_list(
                    &[GeneratorMode::Range, GeneratorMode::CustomList][..],
                    Some(self.mode.clone()),
                    Message::ModeChanged
                )
                .text_size(14)
                .style(move |_theme: &Theme, _status| {
                    pick_list::Style {
                        placeholder_color: if self.dark_mode {
                            Color::from_rgb(0.6, 0.6, 0.6)
                        } else {
                            Color::from_rgb(0.4, 0.4, 0.4)
                        },
                        handle_color: if self.dark_mode {
                            Color::from_rgb(0.7, 0.7, 0.7)
                        } else {
                            Color::from_rgb(0.4, 0.4, 0.4)
                        },
                        text_color: if self.dark_mode {
                            Color::from_rgb(0.9, 0.9, 0.9)
                        } else {
                            Color::BLACK
                        },
                        background: Background::Color(
                            if self.dark_mode {
                                Color::from_rgb(0.25, 0.25, 0.3)
                            } else {
                                Color::WHITE
                            }
                        ),
                        border: Border {
                            color: if self.dark_mode {
                                Color::from_rgb(0.4, 0.4, 0.45)
                            } else {
                                Color::from_rgb(0.8, 0.8, 0.8)
                            },
                            width: 1.0,
                            radius: 6.0.into(),
                        },
                    }
                }),
            ]
                .spacing(6)
                .align_y(alignment::Vertical::Center)
        )
            .padding(2);

        // Range mode inputs - now includes Count
        let range_inputs = if self.mode == GeneratorMode::Range {
            container(
                row![
                    // From input
                    column![
                        text("From").size(14),
                        text_input("", &self.lower_bound)
                            .on_input(Message::LowerBoundChanged)
                            .width(Length::Fixed(60.0))
                            .size(14)
                            .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode))
                    ]
                    .spacing(2),

                    Space::with_width(Length::Fixed(8.0)),

                    // To input
                    column![
                        text("To").size(14),
                        text_input("", &self.upper_bound)
                            .on_input(Message::UpperBoundChanged)
                            .width(Length::Fixed(60.0))
                            .size(14)
                            .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode))
                    ]
                    .spacing(2),

                    Space::with_width(Length::Fixed(8.0)),

                    // Count input
                    column![
                        text("Count").size(14),
                        text_input("", &self.num_to_generate)
                            .on_input(Message::NumToGenerateChanged)
                            .width(Length::Fixed(60.0))
                            .size(14)
                            .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode))
                    ]
                    .spacing(2),
                ]
                    .spacing(6)
                    .align_y(alignment::Vertical::Bottom)
            )
        } else {
            container(Space::with_width(Length::Fixed(0.0)))
        };

        // Custom list mode input
        let custom_list_input = if self.mode == GeneratorMode::CustomList {
            container(
                column![
                    text("Numbers (comma/space separated):").size(14),
                    text_input("e.g. 1, 2, 3, 4, 5", &self.custom_list_input)
                        .on_input(Message::CustomListChanged)
                        .width(Length::Fill)
                        .size(14)
                        .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode)),
                    Space::with_height(Length::Fixed(4.0)),
                    // Count input for custom list mode
                    row![
                        column![
                            text("Count").size(14),
                            text_input("", &self.num_to_generate)
                                .on_input(Message::NumToGenerateChanged)
                                .width(Length::Fixed(60.0))
                                .size(14)
                                .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode))
                        ]
                        .spacing(2),
                    ]
                ]
                    .spacing(4)
            )
                .padding(4)
        } else {
            container(Space::with_height(Length::Fixed(0.0)))
        };

        let input_section = container(
            column![
                mode_picker,
                horizontal_rule(1).style(move |_theme: &Theme| {
                    iced::widget::rule::Style {
                        color: if self.dark_mode {
                            Color::from_rgb(0.4, 0.4, 0.45)
                        } else {
                            Color::from_rgb(0.8, 0.8, 0.8)
                        },
                        width: 1,
                        radius: 0.0.into(),
                        fill_mode: iced::widget::rule::FillMode::Full,
                    }
                }),
                range_inputs,
                custom_list_input,
                Space::with_height(Length::Fixed(6.0)),

                // Checkbox
                checkbox("Allow duplicates", self.generator.get_allow_duplicates())
                    .on_toggle(Message::AllowDuplicatesToggled)
                    .size(14)
                    .text_size(14)
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
                .spacing(6)
                .padding(10)
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

        // Button row with filename input
        let button_row = row![
            button(text("Generate").size(14))
                .on_press(Message::Generate)
                .width(Length::Fixed(85.0))
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

            button(text("Clear").size(14))
                .on_press(Message::Clear)
                .width(Length::Fixed(65.0))
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

            button(text("Save").size(14))
                .on_press(Message::Save)
                .width(Length::Fixed(65.0))
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
                }),

            Space::with_width(Length::Fixed(8.0)),

            // Filename input
            text("File:").size(14),
            text_input("", &self.filename)
                .on_input(Message::FilenameChanged)
                .width(Length::Fill)
                .size(14)
                .style(move |_theme: &Theme, _status| get_text_input_style(self.dark_mode))
        ]
            .spacing(6)
            .align_y(alignment::Vertical::Center);

        let error_display = if !self.error_message.is_empty() {
            container(
                text(&self.error_message)
                    .size(13)
                    .style(move |_theme: &Theme| {
                        iced::widget::text::Style {
                            color: Some(if self.error_message.starts_with("Saved") {
                                Color::from_rgb(0.4, 0.8, 0.4)
                            } else {
                                Color::from_rgb(1.0, 0.4, 0.4)
                            }),
                        }
                    })
            )
                .padding(4)
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
                text(match self.mode {
                    GeneratorMode::Range => "Click Generate to start",
                    GeneratorMode::CustomList => "Enter numbers and click Generate",
                })
                    .size(14)
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
            let chunk_size = 8;

            let mut rows = Vec::new();
            for chunk in numbers.chunks(chunk_size) {
                let number_row = row(
                    chunk.iter().map(|num| {
                        container(
                            text(format!("{}", num))
                                .size(13)
                                .font(iced::Font::MONOSPACE)
                        )
                            .padding(3)
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
                    .spacing(3);
                rows.push(number_row.into());
            }

            // Add total count
            rows.push(Space::with_height(Length::Fixed(6.0)).into());
            rows.push(
                container(
                    text(format!("Total: {}", numbers.len()))
                        .size(13)
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
                        .spacing(3)
                        .padding(6)
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
            button(text("About")
                .size(13))
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
                .size(12)
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
            Space::with_height(Length::Fixed(10.0)),
            input_section,
            Space::with_height(Length::Fixed(10.0)),
            button_row,
            Space::with_height(Length::Fixed(6.0)),
            error_display,
            Space::with_height(Length::Fixed(10.0)),
            results_display,
            Space::with_height(Length::Fill),
            status_bar
        ]
            .spacing(0)
            .padding(14);

        if self.about_open {
            let about_content = container(
                column![
                    text("Random Generator")
                        .size(20)
                        .color(if self.dark_mode { Color::from_rgb(0.9, 0.9, 0.9) } else { Color::BLACK }),
                    Space::with_height(Length::Fixed(10.0)),
                    text(format!("GUI: {}", self.gui_version))
                        .size(14),
                    text(format!("Core: {}", self.generator.get_core_version()))
                        .size(14),
                    Space::with_height(Length::Fixed(14.0)),
                    text("GitHub: https://github.com/Daihongyi/random-tool-github")
                        .size(12),
                    Space::with_height(Length::Fixed(10.0)),
                    text("License: MPL-2.0")
                        .size(12),
                    text("Built with Rust")
                        .size(12),
                    text("Powered by Iced")
                        .size(12),
                    Space::with_height(Length::Fixed(18.0)),
                    button(text("Close").size(14))
                        .on_press(Message::CloseAbout)
                        .width(Length::Fixed(80.0))
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
                    .spacing(4)
                    .align_x(alignment::Horizontal::Center)
                    .padding(24)
            )
                .center_x(Length::Fixed(300.0))
                .center_y(Length::Fixed(260.0))
                .width(Length::Fixed(300.0))
                .height(Length::Fixed(260.0))
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

// Define function to get text input style
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
        RandomGeneratorApp::view,
    )
        .theme(RandomGeneratorApp::theme)
        .window(iced::window::Settings {
            size: iced::Size::new(400.0, 400.0),
            position: Default::default(),
            min_size: Some(iced::Size::new(300.0, 400.0)),
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