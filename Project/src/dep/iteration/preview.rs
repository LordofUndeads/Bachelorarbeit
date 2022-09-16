use super::super::modules::{geometry::{Line, Circle},message::PageMessage};

use iced::{
    canvas::event::{self, Event, },
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke, Fill},
    mouse, Point, Rectangle, Column, Length, Element, Size, Alignment
};

#[derive(Default)]
pub struct PreviewState {
    pending: Option<Pending>,
    cache: canvas::Cache,
}

pub struct PreviewPanel {
   pub polygon: PreviewState,
   pub vertices: Vec<Point>,
   pub ignore_input: bool,
}

impl<'a> PreviewPanel {
    pub fn new() -> PreviewPanel {
        PreviewPanel { 
            polygon: PreviewState { 
                pending: None, 
                cache: canvas::Cache::new() }, 
            vertices: vec![],
            
            ignore_input: true,
        }
    }

    pub fn preview_panel(preview_panel: &'a mut PreviewPanel, dark_mode: bool) -> Column<'a, PageMessage>{
        Column::new()
        .padding(0)
        .spacing(0)
        .align_items(Alignment::Center)
        .push(preview_panel.polygon.view((preview_panel.vertices).to_vec(), preview_panel.ignore_input).map(PageMessage::AddPoint))
        
        .into()
    }
}

impl PreviewState {
    pub fn view<'a>(&'a mut self,  vertices: Vec<Point>, ignore_input: bool, ) -> Element<'a, Point> {
        Canvas::new(PreviewPolygonOutLine {
            state: self,
            vertices,
            ignore_input, 
        })
        .width(Length::Units(1280))
        .height(Length::Units(500))
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }

    pub fn set_pending_none(&mut self) {
        self.pending = None;
    }
}

struct PreviewPolygonOutLine<'a> {
   pub state: &'a mut PreviewState,
   pub vertices: Vec<Point>,
   
   pub ignore_input: bool,
}

impl<'a> canvas::Program<Point> for PreviewPolygonOutLine<'a> {
    fn update(&mut self, event: Event, bounds: Rectangle, cursor: Cursor) -> (event::Status, Option<Point>) {
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
        //     Event::Mouse(mouse_hover) => {
        //         let message = match mouse_hover {
        //             mouse::Event::CursorMoved { position } => {
        //                 match self.state.pending {
        //                     None => {
        //                        self.over_first_vertex = check_vertex_bounds(self.vertices[0], position);
                                
        //                         None
        //                     }
        //                     Some(Pending::WaitSndInput { .. } ) => {
        //                         None
        //                     }
        //                     Some(Pending::LoopScdInput { .. }) => {
        //                         None
        //                     }
        //                     Some(Pending::ClipToStartVertex { .. }) => {
        //                         None
        //                     }
        //                     Some(Pending::ConnectToLastVertex { .. }) => {
        //                         None
        //                     }

        //                 }
        //             } _ => None
                    
        //         };
        //         (event::Status::Captured, message)
        //     } 


        Event::Mouse(mouse_event) => {
            let message = match mouse_event {
                    
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    match self.state.pending {
                        None => {
                            
                            

                            None
                        }
                        Some(_) => {
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
   Iterate,
   WaitSelection,
}

impl Pending {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            match *self {

                // Pending::WaitSndInput { from } => {
                    
                //     let circle = Path::circle(from, 1.0);
                //     let line = Path::line(from, cursor_position);
                //     frame.stroke(&line, Stroke::default().with_width(2.0));
                //     frame.stroke(&circle, Stroke::default().with_width(2.0));
                    
                // }
                    
                    Pending::Iterate => {

                    }

                    Pending::WaitSelection => {

                    }
                
                
            };
        }

        frame.into_geometry()
    }
}

