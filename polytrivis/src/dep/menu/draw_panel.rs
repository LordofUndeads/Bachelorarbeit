use super::super::modules::{geometry::{Line, Circle},message::PageMessage};

use iced::{
    canvas::event::{self, Event, },
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke, Fill},
    mouse, Point, Rectangle, Column, Length, Element,  Color
};



#[derive(Default)]
pub struct DrawState {
    pending: Option<Pending>,
    cache: canvas::Cache,
}

pub struct DrawPanel {
   pub polygon: DrawState,
   pub vertices: Vec<Point>,
   pub panel_width: u16,
   pub panel_height: u16,
   pub closed: bool,
   pub ignore_input: bool,
}

impl<'a> DrawPanel {
    pub fn new() -> DrawPanel {
        DrawPanel { 
            polygon: DrawState { 
                pending: None, 
                cache: canvas::Cache::new() }, 
            vertices: vec![],
            panel_width: 600,
            panel_height: 400,
            closed: false,
            ignore_input: true,
        }
    }

    pub fn draw_panel(draw_panel: &'a mut DrawPanel, ) -> Column<'a, PageMessage>{
        Column::new()
        .padding(0)
        .spacing(0)
  
        .push(draw_panel.polygon.view((draw_panel.vertices).to_vec(), draw_panel.ignore_input,  draw_panel.panel_width,
            draw_panel.panel_height, draw_panel.closed,).map(PageMessage::AddPoint))
        
        .into()
    }
}

impl DrawState {
    pub fn view<'a>(&'a mut self,  vertices: Vec<Point>, ignore_input: bool, panel_width: u16, panel_height: u16 ,closed: bool) -> Element<'a, Point> {
        Canvas::new(PolygonOutLine {
            state: self,
            vertices,
            closed,
            ignore_input, 
        })
        .width(Length::Units(panel_width))
        .height(Length::Units(panel_height))
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }

    pub fn set_pending_none(&mut self) {
        self.pending = None;
    }

    pub fn set_pending_waitinput(&mut self, vertex: Point) {
        self.pending = Some(Pending::WaitNxtInput { from: (vertex)});
    }
}

struct PolygonOutLine<'a> {
   pub state: &'a mut DrawState,
   pub vertices: Vec<Point>,
   pub closed: bool,
   pub ignore_input: bool,
}

impl<'a> canvas::Program<Point> for PolygonOutLine<'a> {
    fn update(&mut self, event: Event, bounds: Rectangle, cursor: Cursor) -> (event::Status, Option<Point>) {
        
        let mut over_first_vertex = false;
        
        let cursor_position =
            if self.ignore_input {
                return (event::Status::Ignored, None);
            }
            else if let Some(position) = cursor.position_in(&bounds){
                
                position
             
            } else {
                return (event::Status::Ignored, None);
            };

            if self.vertices.len() > 1 {
                if check_vertex_bounds(self.vertices[0], cursor_position) {
                    over_first_vertex = true;
                }
            }

         match event {

            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                        
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match self.state.pending {
                            None => {
                               
                               
                                self.state.pending = Some(Pending::WaitNxtInput {
                                        from: cursor_position
                                                        });
                               
                                Some(Point {
                                    x: cursor_position.x,
                                    y: cursor_position.y
                                })
                            }
                            Some(Pending::WaitNxtInput { .. }) => {

                                if over_first_vertex {

                                    if let Some(from) = self.vertices.first() {
                                        if let Some(to) = self.vertices.last() { 
                                            
                                            self.closed = true;
                                            self.ignore_input = true;
                                            self.state.request_redraw();
                                            self.state.pending = Some(Pending::ClipToStartPoint{
                                                last: *from, 
                                                first: *to
                                            });
                                            
                                        }
                                        
                                    }  
                                    
                                    None
                                   
                                }
                                else {
                                    self.state.pending = Some(Pending::WaitNxtInput {
                                    
                                        from: cursor_position,
                                    });
                                    
                                    Some(Point {
                                        x: cursor_position.x,
                                        y: cursor_position.y
                                    })
                                    
                                }
                                

                           
                            }

                            Some(Pending::ClipToStartPoint {   ..  } )=> {
                                
                                None
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
                Circle::draw_all(&self.vertices, 3.0,frame);
                if self.vertices.len() > 2 && self.closed == false {
                    Circle::draw_unfilled(self.vertices[0], 5.0, frame);
                }
                if self.closed {
                    if let Some(from) = self.vertices.first() {
                        if let Some(to) = self.vertices.last() { 
                            Line::draw(*from, *to, frame);}}   
                }
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
    WaitNxtInput { from: Point },
    ClipToStartPoint {last: Point, first: Point},
   
}

impl Pending {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            match *self {

                Pending::WaitNxtInput { from } => {
                    
                    let circle = Path::circle(from, 3.0);
                    let line = Path::line(from, cursor_position);
                    frame.stroke(&line, Stroke::default().with_width(2.0));
                    frame.fill(&circle, Fill { color: Color::BLACK, rule: canvas::FillRule::NonZero});

                    let vertices: Vec<Point> = vec![from, cursor_position];              

                    Circle::draw_all(&vertices,3.0, &mut frame);
                    Line::draw_all(&vertices, &mut frame);
                    
                }

                Pending::ClipToStartPoint {last, first }=> {

                            let line = Path::line(last,first);
                            frame.stroke(&line, Stroke::default().with_width(2.0));   
                   
                }

            };
        }

        frame.into_geometry()
    }
}

// helper function to check if the cursor is in a square around a vertex of the polygon or not
fn check_vertex_bounds(vertex: Point, cursor_position: Point) -> bool {
    
    if cursor_position.x > vertex.x - 4.0 && cursor_position.x < vertex.x + 4.0 {
        if cursor_position.y > vertex.y - 4.0 && cursor_position.y < vertex.y + 4.0 {
            return true;
        }
        else { return false; }
    }
    else { return false; }
   
}