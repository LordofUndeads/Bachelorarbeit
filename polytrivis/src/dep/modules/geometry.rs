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
    pub fn draw_all_line(lines: &Vec<Line>, frame: &mut Frame) {
        
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
    pub fn draw_all_vertex(vertices: &Vec<Vertex>, radius: f32,frame: &mut Frame) {
        let circles = Path::new(|p| {
            for vertex in vertices{
                p.circle(Point::new(vertex.x, vertex.y), radius);
                Circle::draw(Point::new(vertex.x, vertex.y), radius, frame)
            }
           });
           frame.fill(&circles, Fill::default());
        
    }
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

pub struct BoundingBox {
    pub ankor: Point,
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn new(ankor: Point, width: f32, height: f32) -> BoundingBox {
        BoundingBox { ankor, width , height }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub id: u16,
    pub signum: char,
}

impl Vertex {
    pub fn new(point: Point, id: u16) -> Vertex {
        Vertex {
            x: point.x,
            y: point.y,
            id,
            signum: '+',
        }
    }

    pub fn new_with_sig(point: Point, id: u16, signum: char) -> Vertex {
        Vertex {
            x: point.x,
            y: point.y,
            id,
            signum
        }
    }

    pub fn without_id(vertices: Vec<Vertex>) -> Vec<Point> {
        let mut buffer: Vec<Point> = vec![];

        for v in vertices {
            buffer.push(Point::new(v.x, v.y))
        }
         return buffer;
    }

    pub fn convert_to_vertex_buffer(points: Vec<Point>) -> Vec<Vertex> {
        
        let mut vertices: Vec<Vertex> = vec![];

        for i in 0..points.len() {
            vertices.push(Vertex::new(points[i], i as u16))
        }

        return vertices;
    }
}