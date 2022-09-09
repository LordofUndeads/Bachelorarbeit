use iced::{
   
    canvas::{ Frame, Path, Stroke, Fill},
    Point,
};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
   pub from: Point,
   pub to: Point,

}

#[derive(Debug, Clone)]
pub struct Circle{
   // center: Point,
}

impl Line {
    pub fn draw_all(lines: &[Line], frame: &mut Frame) {
        let lines = Path::new(|p| {
            for line in lines {
                p.move_to(line.from);
                p.line_to(line.to);
            }
        });

        frame.stroke(&lines, Stroke::default().with_width(2.0));
    }
}

impl Circle {
    pub fn draw_all(circles: &[Point], radius: f32,frame: &mut Frame) {
        let circles = Path::new(|p| {
            for vertex in circles{
                p.circle(*vertex, radius)
            }
           });
           frame.fill(&circles, Fill::default());
        
    }
}
