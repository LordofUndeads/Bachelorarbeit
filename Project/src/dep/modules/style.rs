use iced::{button, Button};
use iced::{Background, Color, Vector, Text, alignment, Length};
use iced::rule;


#[derive(Clone, Debug)]
    pub enum ButtonStyle {
        Primary,
        Secondary,
    }


    impl<'a> button::StyleSheet for ButtonStyle {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    ButtonStyle::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    ButtonStyle::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
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

    pub enum Rule {
        Ligth,
        //Dark,
    }

    impl rule::StyleSheet for Rule {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: match self {
                    Rule::Ligth => Color::from_rgb(0.5, 0.5, 0.5),
                    //Rule::Dark => Color::from_rgb(0.5, 0.5, 0.5),
                },
                ..rule::Style::default()
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