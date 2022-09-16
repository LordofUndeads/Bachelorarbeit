use iced::{
   
    canvas::{ Frame, Path, Stroke, Fill, FillRule},
    Point, Color
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

    pub fn new(from: Point, to: Point) -> Line {
        Line {
            from,
            to,
        }
    }

    pub fn draw_all(vertices: &Vec<Point>, frame: &mut Frame) {
        
        let lines = Line::make_lines(vertices);
        
        let outlines = Path::new(|p| {
            for line in lines {
                p.move_to(line.from);
                p.line_to(line.to);
            }
        });

        frame.stroke(&outlines, Stroke::default().with_width(2.0));
    }

    fn make_lines(vertices: &Vec<Point>) -> Vec<Line> {
        let mut lines: Vec<Line> = vec![];
        for i in vertices.windows(2) {
            lines.push(Line::new( i[0], i[1]))
        }
        return lines;
    }

    pub fn draw(from: Point, to: Point, frame: &mut Frame) {
        frame.stroke(&Path::line(from, to), Stroke::default().with_width(2.0));
    }
}

impl Circle {
    pub fn draw_all(vertices: &Vec<Point>, radius: f32,frame: &mut Frame) {
        let circles = Path::new(|p| {
            for vertex in vertices{
                p.circle(*vertex, radius);
                Circle::draw(*vertex, radius, frame)
            }
           });
           frame.fill(&circles, Fill::default());
        
    }

    pub fn draw(vertex: Point, radius: f32, frame: &mut Frame) {
        frame.fill(&Path::circle(vertex, radius), Fill {color: Color::BLACK, rule: FillRule::NonZero });
    }

    pub fn draw_unfilled(vertex: Point, radius: f32, frame: &mut Frame) {
        frame.stroke(&Path::circle(vertex, radius), Stroke::default().with_width(2.0));
    }
}
