use super::super::modules::{geometry::{Line, Circle},message::PageMessage};

use iced::{
    canvas::event::{self, Event, },
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke, Fill},
    mouse, Point, Rectangle, Column, Length, Element, Size
};

#[derive(Default)]
pub struct DrawState {
    pending: Option<Pending>,
    cache: canvas::Cache,
}

pub struct DrawPanel {
   pub polygon: DrawState,
   pub vertices: Vec<Point>,
   pub ignore_input: bool,
}

impl<'a> DrawPanel {
    pub fn new() -> DrawPanel {
        DrawPanel { 
            polygon: DrawState { 
                pending: None, 
                cache: canvas::Cache::new() }, 
            vertices: vec![],
            ignore_input: true,
        }
    }

    pub fn draw_panel(draw_panel: &'a mut DrawPanel) -> Column<'a, PageMessage>{
        Column::new()
        .padding(0)
        .spacing(0)
        
        
        .push(draw_panel.polygon.view((draw_panel.vertices).to_vec(), draw_panel.ignore_input).map(PageMessage::AddLine))
        
        .into()
    }
}

impl DrawState {
    pub fn view<'a>(&'a mut self,  vertices: Vec<Point>, ignore_input: bool) -> Element<'a, Line> {
        Canvas::new(PolygonOutLine {
            state: self,
            
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

    pub fn set_pending_none(&mut self) {
        self.pending = None;
    }
}

struct PolygonOutLine<'a> {
   pub state: &'a mut DrawState,
   pub vertices: Vec<Point>,
   
   pub ignore_input: bool,
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
                                
                                self.state.pending = Some(Pending::WaitSndInput {
                                    from: cursor_position,
                                });

                                None
                            }
                            Some(Pending::WaitSndInput { from }) => {
                                self.state.pending = Some(Pending::LoopScdInput {
                                    from,
                                    to: cursor_position,
                                });

                               None
                            }
                            Some(Pending::LoopScdInput { from, to }) => {
                                self.state.pending = Some(Pending::LoopScdInput {
                                    from: to,
                                    to: cursor_position,
                                });
                                Some(Line {
                                    from,
                                    to,
                                })
                            }

                            Some(Pending::ClipToStartVertex { connector }) => {
                                self.state.pending= Some(Pending::LoopScdInput { 
                                    from: connector, 
                                    to: cursor_position });
                                Some(Line {
                                    from: connector,
                                    to: cursor_position,
                                })
                            }

                            Some(Pending::ConnectToLastVertex { from }) => {
                                
                                Some(Line {
                                    from,
                                    to: cursor_position,
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
                Line::draw_all(&self.vertices, frame);
                Circle::draw_all(&self.vertices, 1.0,frame);
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
    WaitSndInput { from: Point },
    ClipToStartVertex { connector: Point},
    LoopScdInput { from: Point, to: Point },
    ConnectToLastVertex {from: Point},

}

impl Pending {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            match *self {

                Pending::WaitSndInput { from } => {
                    
                    let circle = Path::circle(from, 1.0);
                    let line = Path::line(from, cursor_position);
                    frame.stroke(&line, Stroke::default().with_width(2.0));
                    frame.stroke(&circle, Stroke::default().with_width(2.0));
                    
                }

                Pending::LoopScdInput { from, to } => {
                    let line = Path::line(to, cursor_position);
                    let circle = Path::circle(to, 1.0);
                    frame.stroke(&line, Stroke::default().with_width(2.0));
                    frame.stroke(&circle, Stroke::default().with_width(2.0));
                    
                    let vertices: Vec<Point> = vec![from, to];              

                    Circle::draw_all(&vertices,1.0, &mut frame);
                    Line::draw_all(&vertices, &mut frame);
                }

                Pending::ClipToStartVertex { connector } => {
                    let circle = Path::circle(connector, 3.0);
                    frame.stroke(&circle, Stroke::default().with_width(2.0));
                }

                Pending::ConnectToLastVertex { from } => {

                }
                
            };
        }

        frame.into_geometry()
    }
}

// helper function to check if the cursor is in a square around a vertex of the polygon or not
fn check_vertex_bounds(vertex: Point, cursor_position: Point) -> bool {
    
    if cursor_position.x > vertex.x - 0.5 && cursor_position.x < vertex.x + 0.5 {
        if cursor_position.y > vertex.y - 0.5 && cursor_position.y < vertex.y + 0.5 {
            return true;
        }
        else { return false; }
    }
    else { return false; }
   
}