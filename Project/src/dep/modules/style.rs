use iced::{button, Button};
use iced::{Background, Color, Vector, Text, alignment, Length};
use iced::rule;
use iced::container;

use iced_aw::card;



#[derive(Clone, Debug)]
    pub enum ButtonStyle {
        PrimaryLight,
        PrimaryDark,
        SecondaryLight,
        SecondaryDark,
        Green,
        Red,
    }


    impl<'a> button::StyleSheet for ButtonStyle {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    //ButtonStyle::PrimaryLight => Color::from_rgb(0.11, 0.42, 0.87),
                    ButtonStyle::PrimaryLight => Color::from_rgb(0.11, 0.42, 0.87),
                    ButtonStyle::PrimaryDark => Color::from_rgb(0.11, 0.42, 0.87),
                    //ButtonStyle::SecondaryLight => Color::from_rgb(0.5, 0.5, 0.5),
                    ButtonStyle::SecondaryLight => Color::from_rgb(0.5, 0.5, 0.5),
                    ButtonStyle::SecondaryDark => Color::from_rgb(0.5, 0.5, 0.5),
                    ButtonStyle::Green => Color::from_rgb8(0x32, 0xcd, 0x32),
                    ButtonStyle::Red => Color::from_rgb8(0x8b, 0x00, 0x00)
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                    // match self {
                    //     ButtonStyle::PrimaryLight => Color::from_rgb8(0xc6, 0xc6, 0xc6),
                    //     ButtonStyle::PrimaryDark => Color::from_rgb8(0x84, 0x81, 0x83),
                    //     ButtonStyle::SecondaryLight => Color::from_rgb8(0xc6, 0xc6, 0xc6),
                    //     ButtonStyle::SecondaryDark => Color::from_rgb8(0x84, 0x81, 0x83),
                    // },
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }

        
    }

    pub enum RuleStyle {
        Light,
        Dark,
    }

    impl rule::StyleSheet for RuleStyle {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: match self {
                    RuleStyle::Light => Color::from_rgb(0.5, 0.5, 0.5),
                    RuleStyle::Dark => Color::from_rgb(0.5, 0.5, 0.5),
                },
                ..rule::Style::default()
            }

        }
    }

    pub enum WindowStyle {
        Light,
        Dark
    }

    impl container::StyleSheet for WindowStyle {
        fn style(&self) -> container::Style {
            container::Style { 
                
                background:  Some(Background::Color(match self {
                    WindowStyle::Light => Color::WHITE,
                    WindowStyle::Dark => Color::from_rgb8(0x57, 0x57, 0x57),
                })),
                text_color: match self {
                    WindowStyle::Light => Some(Color::from_rgb8(0x57, 0x57, 0x57)),
                    WindowStyle::Dark => Some(Color::WHITE),
                },
                ..container::Style::default()
            }
        
        }
    }

    pub enum PopUpStyle {
        Light,
        Dark,
    }

    impl card::StyleSheet for PopUpStyle {
        fn active(&self) -> card::Style {
            card::Style { 
                background: Background::Color(match self {
                    PopUpStyle::Light => Color::WHITE,
                    PopUpStyle::Dark => Color::from_rgb8(0x57, 0x57, 0x57), 
                    }), 
                // border_radius: (), 
                // border_width: (), 
                border_color: match self {
                    PopUpStyle::Light => Color::from_rgb(0.5, 0.5, 0.5),
                    PopUpStyle::Dark => Color::from_rgb(0.5, 0.5, 0.5),
                }, 
                // head_background: (), 
                // head_text_color: (), 
                // body_background: ().
                // body_text_color: (), 
                // foot_background: (), 
                // foot_text_color: (), 
                // close_color: () }
                ..card::Style::default()
                }
        }
    }
    //helper functions for gui elements

    pub fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str,) -> Button<'a, Message> {
        Button::new(
            state,
            Text::new(label).horizontal_alignment(alignment::Horizontal::Center).size(15),
        )
        .padding(10)
        .width(Length::Units(100))
    }