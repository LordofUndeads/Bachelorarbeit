use iced::{
    button, Alignment, Button, Column, Element, Length, Sandbox, Settings, Text,
};

pub fn main() -> iced::Result {
    Example::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
struct Example {
    polygon: polygon::State,
    curves: Vec<polygon::Line>,
    button_state: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AddCurve(polygon::Line),
    Clear,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example::default()
    }

    fn title(&self) -> String {
        String::from("Polygon Draw - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddCurve(curve) => {
                self.curves.push(curve);
                self.polygon.request_redraw();
            }
            Message::Clear => {
                self.polygon = polygon::State::default();
                self.curves.clear();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                Text::new("Bezier tool example")
                    .width(Length::Shrink)
                    .size(50),
            )
            .push(self.polygon.view(&self.curves).map(Message::AddCurve))
            .push(
                Button::new(&mut self.button_state, Text::new("Clear"))
                    .padding(8)
                    .on_press(Message::Clear),
            )
            .into()
    }
}

mod polygon {
    use iced::{
        canvas::event::{self, Event},
        canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke},
        mouse, Element, Length, Point, Rectangle,
    };

    #[derive(Default)]
    pub struct State {
        pending: Option<Pending>,
        cache: canvas::Cache,
    }

    impl State {
        pub fn view<'a>(
            &'a mut self,
            lines: &'a [Line],
        ) -> Element<'a, Line> {
            Canvas::new(PolygonLine {
                state: self,
                lines,
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }

        pub fn request_redraw(&mut self) {
            self.cache.clear()
        }
    }

    struct PolygonLine<'a> {
        state: &'a mut State,
        lines: &'a [Line],
    }

    impl<'a> canvas::Program<Line> for PolygonLine<'a> {
        fn update(
            &mut self,
            event: Event,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> (event::Status, Option<Line>) {
            let cursor_position =
                if let Some(position) = cursor.position_in(&bounds) {
                    position
                } else {
                    return (event::Status::Ignored, None);
                };

            match event {
                Event::Mouse(mouse_event) => {
                    let message = match mouse_event {
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            match self.state.pending {
                                None => {
                                    self.state.pending = Some(Pending::One {
                                        from: cursor_position,
                                    });

                                    None
                                }
                                Some(Pending::One { from }) => {
                                    self.state.pending = Some(Pending::Two {
                                        from,
                                        to: cursor_position,
                                    });

                                    None
                                }
                                Some(Pending::Two { from, to }) => {
                                    self.state.pending = Some(Pending::Two {
                                        from: to,
                                        to: cursor_position,
                                    });
                                    Some(Line {
                                        from,
                                        to,
                                    })
                                }
                                
                            }
                        }
                        _ => None,
                    };

                    (event::Status::Captured, message)
                }
                _ => (event::Status::Ignored, None),
            }
        }

        fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
            let content =
                self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
                    Line::draw_all(self.lines, frame);

                    frame.stroke(
                        &Path::rectangle(Point::ORIGIN, frame.size()),
                        Stroke::default(),
                    );
                });

            if let Some(pending) = &self.state.pending {
                let pending_curve = pending.draw(bounds, cursor);

                vec![content, pending_curve]
            } else {
                vec![content]
            }
        }

        fn mouse_interaction(
            &self,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> mouse::Interaction {
            if cursor.is_over(&bounds) {
                mouse::Interaction::Crosshair
            } else {
                mouse::Interaction::default()
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Line {
        from: Point,
        to: Point,
  
    }

    impl Line {
        fn draw_all(lines: &[Line], frame: &mut Frame) {
            let lines = Path::new(|p| {
                for line in lines {
                    p.move_to(line.from);
                    p.line_to(line.to);
                }
            });

            frame.stroke(&lines, Stroke::default().with_width(2.0));
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum Pending {
        One { from: Point },
        Two { from: Point, to: Point },
  
    }

    impl Pending {
        fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
            let mut frame = Frame::new(bounds.size());

            if let Some(cursor_position) = cursor.position_in(&bounds) {
                match *self {
                    Pending::One { from } => {
                        let line = Path::line(from, cursor_position);
                        frame.stroke(&line, Stroke::default().with_width(2.0));
                    }
                    Pending::Two { from, to } => {
                        let line = Path::line(to, cursor_position);
                        frame.stroke(&line, Stroke::default().with_width(2.0));
                        let line = Line {
                            from,
                            to,
                            
                        };

                        Line::draw_all(&[line], &mut frame);
                    }
                   
                };
            }

            frame.into_geometry()
        }
    }
}