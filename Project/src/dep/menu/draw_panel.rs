use super::super::modules::{geometry::{Line, Circle},message::PageMessage};

use iced::{
    canvas::event::{self, Event, },
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke, Fill},
    mouse, Point, Rectangle, Column, Length, Element
};

#[derive(Default)]
pub struct DrawState {
    pending: Option<Pending>,
    cache: canvas::Cache,
}

pub struct DrawPanel {
   pub polygon: DrawState,
   pub lines: Vec<Line>,
   pub vertices: Vec<Point>,
   pub ignore_input: bool,
}

impl<'a> DrawPanel {
    pub fn new() -> DrawPanel {
        DrawPanel { 
            polygon: DrawState { 
                pending: None, 
                cache: canvas::Cache::new() }, 
            lines: vec![], 
            vertices: vec![],
            ignore_input: true,
        }
    }

    pub fn draw_panel(draw_panel: &'a mut DrawPanel) -> Column<'a, PageMessage>{
        Column::new()
        .padding(0)
        .spacing(0)
        
        
        .push(draw_panel.polygon.view(&draw_panel.lines, &draw_panel.vertices, draw_panel.ignore_input).map(PageMessage::AddLine))
        
        .into()
    }
}

impl DrawState {
    pub fn view<'a>(&'a mut self, lines: &'a [Line], vertices: &'a [Point], ignore_input: bool) -> Element<'a, Line> {
        Canvas::new(PolygonOutLine {
            state: self,
            lines,
            vertices,
            ignore_input, 
        })
        .width(Length::Units(600))
        .height(Length::Units(400))
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }
}

struct PolygonOutLine<'a> {
    state: &'a mut DrawState,
    lines: &'a [Line],
    vertices: &'a [Point],
    ignore_input: bool,
}

impl<'a> canvas::Program<Line> for PolygonOutLine<'a> {
    fn update(&mut self, event: Event, bounds: Rectangle, cursor: Cursor) -> (event::Status, Option<Line>) {
        let cursor_position =
            if self.ignore_input {
                return (event::Status::Ignored, None);
            }
            else if let Some(position) = cursor.position_in(&bounds){
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
                Circle::draw_all(self.vertices, 3.0,frame);
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default(),
                );
            });

        if let Some(pending) = &self.state.pending {
            let pending_line = pending.draw(bounds, cursor);

            vec![content, pending_line]
        } else {
            vec![content]
        }
    }

    fn mouse_interaction(
        &self,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(&bounds) && !self.ignore_input {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
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
                    
                    let circle = Path::circle(cursor_position, 3.0);
                    let line = Path::line(from, cursor_position);
                    frame.stroke(&line, Stroke::default().with_width(2.0));
                    frame.stroke(&circle, Stroke::default().with_width(2.0))
                    //Circle::draw_all(&[circle], 3.0, &mut frame)
                }
                Pending::Two { from, to } => {
                    let line = Path::line(to, cursor_position);
                    frame.stroke(&line, Stroke::default().with_width(2.0));
                    
                    let line = Line {
                        from,
                        to,
                        
                    };

                    //  let circle = Circle {center: from, radius: 3.0};
              

                    //Circle::draw_all(&[circle], &mut frame);
                    Line::draw_all(&[line], &mut frame);
                }
                
            };
        }

        frame.into_geometry()
    }
}
