use super::super::modules::{geometry::{Line, Circle},message::PageMessage};

use iced::{
    canvas::event::{self, Event, },
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke,},
    Point, Rectangle, Column, Length, Element,  Alignment
};

#[derive(Default)]
pub struct ResultState {
    cache: canvas::Cache,
}

pub struct ResultPanel {
   pub polygon: ResultState,
   pub vertices: Vec<Point>,
   pub panel_width: u16,
   pub panel_height: u16,
}

impl<'a> ResultPanel {
    pub fn new() -> ResultPanel {
        ResultPanel { 
            polygon: ResultState { 
                cache: canvas::Cache::new() }, 
            vertices: vec![],
            panel_width: 1280,
            panel_height: 500
        }
    }

    pub fn result_panel(result_panel: &'a mut ResultPanel) -> Column<'a, PageMessage>{
        Column::new()
        .padding(0)
        .spacing(0)
        .align_items(Alignment::Center)
        .push(result_panel.polygon.view((result_panel.vertices).to_vec(), result_panel.panel_width, result_panel.panel_height
                ).map(PageMessage::AddPoint))
        
        .into()
    }

   
}

impl ResultState {
    pub fn view<'a>(&'a mut self,  vertices: Vec<Point>, panel_width: u16, panel_height: u16) -> Element<'a, Point> {
        Canvas::new(PreviewPolygonOutLine {
            state: self,
            vertices,
             
        })
        .width(Length::Units(panel_width))
        .height(Length::Units(panel_height))
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }

}

struct PreviewPolygonOutLine<'a> {
   pub state: &'a mut ResultState,
   pub vertices: Vec<Point>,

}

impl<'a> canvas::Program<Point> for PreviewPolygonOutLine<'a> {
    fn update(&mut self, _event: Event, _bounds: Rectangle, _cursor: Cursor) -> (event::Status, Option<Point>) {
        (event::Status::Captured, None)
    
}
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let content =
            self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
                Line::draw_all(&self.vertices, frame);
                Circle::draw_all(&self.vertices, 3.0,frame);
                if let Some(from) = self.vertices.first() {
                    if let Some(to) = self.vertices.last() { 
                        Line::draw(*from, *to, frame);}}  
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default(),
                );
            });

                vec![content]
        
    }


}   




