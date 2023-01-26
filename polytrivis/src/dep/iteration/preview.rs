use super::super::modules::{geometry::{Line, Circle, Vertex},message::PageMessage,};


use iced::{
    canvas::event::{self, Event, },
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke, },
    mouse, Point, Rectangle, Column, Length, Element, Alignment
};

use super::super::super::dep::eca_primitiv::grid::{Grid, Stats};

#[derive(Default)]
pub struct PreviewState {
    pending: Option<Pending>,
    cache: canvas::Cache,
}

pub struct PreviewPanel {
   pub polygon: PreviewState,
   pub vertices: Vec<Vertex>,
   pub diagonals: Vec<Line>,
   pub grid: Grid,
   pub stats: Stats,
   pub panel_width: u16,
   pub panel_height: u16,
   pub ignore_input: bool,

   
}

impl<'a> PreviewPanel {
    pub fn new() -> PreviewPanel {
        PreviewPanel { 
            polygon: PreviewState { 
                pending: None, 
                cache: canvas::Cache::new() }, 
            vertices: vec![],
            diagonals: vec![],
            grid: Grid::new(Point::new(0.0,0.0), 1280, 500, vec![], ),
            stats: Stats::new(),
            panel_width: 1280,
            panel_height: 500,
            ignore_input: true,
            
        }
    }

    pub fn preview_panel(preview_panel: &'a mut PreviewPanel) -> Column<'a, PageMessage>{
        Column::new()
        .padding(0)
        .spacing(0)
        .align_items(Alignment::Center)
        .push(preview_panel.polygon.view((preview_panel.vertices).to_vec(), preview_panel.diagonals.to_vec(),preview_panel.ignore_input,
                preview_panel.panel_width, preview_panel.panel_height).map(PageMessage::AddPoint))
        
        .into()
    }
}

impl PreviewState {
    pub fn view<'a>(&'a mut self,  verts: Vec<Vertex>, diagonals: Vec<Line>,ignore_input: bool, panel_width: u16, panel_height: u16 ) -> Element<'a, Point> {
        Canvas::new(PreviewPolygonOutLine {
            state: self,
            vertices: verts,
            diagonals,
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
}

struct PreviewPolygonOutLine<'a> {
   pub state: &'a mut PreviewState,
   pub vertices: Vec<Vertex>,
   pub diagonals: Vec<Line>,
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
                Line::draw_all(&Vertex::without_id(self.vertices.to_vec()), frame);
                Line::draw_all_line(&self.diagonals, frame);
                Circle::draw_all_vertex(&self.vertices, 3.0,frame);
                if let Some(from) = self.vertices.first() {
                    if let Some(to) = self.vertices.last() { 
                        Line::draw(Point::new(from.x , from.y), Point::new(to.x, to.y), frame);}}   
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

