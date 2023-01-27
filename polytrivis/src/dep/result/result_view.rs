use super::super::modules::{geometry::{Line, Circle, Vertex},message::PageMessage};

use iced::{
    canvas::event::{self, Event, },
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke,},
    Point, Rectangle, Column, Length, Element,  Alignment
};

use super::super::super::dep::eca_primitiv::grid::Stats;

#[derive(Default)]
pub struct ResultState {
    cache: canvas::Cache,
}

pub struct ResultPanel {
   pub polygon: ResultState,
   pub vertices: Vec<Vertex>,
   pub diagonals: Vec<Line>,
   pub stats: Stats,
   pub panel_width: u16,
   pub panel_height: u16,
}

impl<'a> ResultPanel {
    pub fn new() -> ResultPanel {
        ResultPanel { 
            polygon: ResultState { 
                cache: canvas::Cache::new() }, 
            vertices: vec![],
            diagonals: vec![],
            stats: Stats::new(),
            panel_width: 400,
            panel_height: 400
        }
    }

    pub fn result_panel(result_panel: &'a mut ResultPanel) -> Column<'a, PageMessage>{
        Column::new()
        .padding(0)
        .spacing(0)
        .align_items(Alignment::Center)
        .push(result_panel.polygon.view((result_panel.vertices).to_vec(), result_panel.diagonals.to_vec(),result_panel.panel_width, result_panel.panel_height
                ).map(PageMessage::AddPoint))
        
        .into()
    }

   
}

impl ResultState {
    pub fn view<'a>(&'a mut self,  vertices: Vec<Vertex>, diagonals: Vec<Line>,panel_width: u16, panel_height: u16) -> Element<'a, Point> {
        Canvas::new(PreviewPolygonOutLine {
            state: self,
            vertices,
            diagonals,
             
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
   pub vertices: Vec<Vertex>,
   pub diagonals: Vec<Line>,

}

impl<'a> canvas::Program<Point> for PreviewPolygonOutLine<'a> {
    fn update(&mut self, _event: Event, _bounds: Rectangle, _cursor: Cursor) -> (event::Status, Option<Point>) {
        (event::Status::Captured, None)
    
}
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let content =
            self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
                Line::draw_all(&Vertex::without_id(self.vertices.to_vec()), frame);
                Line::draw_all_line(&self.diagonals, self.diagonals.len() ,frame);
                Circle::draw_all_vertex(&self.vertices, 3.0,frame);
                if let Some(from) = self.vertices.first() {
                    if let Some(to) = self.vertices.last() { 
                        Line::draw(Point::new(from.x, from.y), Point::new(to.x , to.y), frame);}}  
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default(),
                );
            });

                vec![content]
        
    }


}   




