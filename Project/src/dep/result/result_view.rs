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
 
}

impl<'a> ResultPanel {
    pub fn new() -> ResultPanel {
        ResultPanel { 
            polygon: ResultState { 
                cache: canvas::Cache::new() }, 
            vertices: vec![],
        }
    }

    pub fn result_panel(result_panel: &'a mut ResultPanel) -> Column<'a, PageMessage>{
        Column::new()
        .padding(0)
        .spacing(0)
        .align_items(Alignment::Center)
        .push(result_panel.polygon.view((result_panel.vertices).to_vec()).map(PageMessage::AddPoint))
        
        .into()
    }
}

impl ResultState {
    pub fn view<'a>(&'a mut self,  vertices: Vec<Point>, ) -> Element<'a, Point> {
        Canvas::new(PreviewPolygonOutLine {
            state: self,
            vertices,
             
        })
        .width(Length::Units(1280))
        .height(Length::Units(500))
        .into()
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
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default(),
                );
            });

                vec![content]
        
    }


}   




