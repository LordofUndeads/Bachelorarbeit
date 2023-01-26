use iced::{Point};
use num_traits::PrimInt;
use round::round_up;
use super::super::modules::geometry::Vertex;


pub struct Grid {
    pub ankor: Point,
    pub width: u16,
    pub height: u16,
    pub vertices: Vec<Vertex>,
    pub reflex_verts: Vec<Vertex>,
    pub grid_segments: Vec<GridSegment>, // width * sqrt(vertecies) x height * sqrt(vertecies)
}

#[derive(PartialEq)]
pub struct GridSegment {
    pub ankor: Point,
    pub width: u16,
    pub height: u16,
    pub state: SegState,
    pub id_list: Vec<u16>,
}
#[derive(PartialEq)]
pub enum SegState {
    Reflex, None
}

pub struct Stats {
    pub min_angles: Vec<f32>,
    pub areas: Vec<f32>,
}

impl<'a> Grid {
    pub fn new(ankor: Point, width: u16, height: u16, vertices: Vec<Vertex>, ) -> Grid {
        Grid {
            ankor,
            width,
            height,
            vertices,
            reflex_verts: vec![],
            grid_segments: vec![],
        }
    }

    pub fn init_segments(grid: &mut Grid ) -> Vec<GridSegment>{

        println!("{}", grid.reflex_verts.len());
        let num_of_verts = grid.vertices.len() as f32;
        
        let n = round_up(num_of_verts.sqrt() as f64, 0) as u16;
        let num_x= grid.width/50 * n;
        let num_y = grid.height/50 * n;

        let seg_width = grid.width / num_x;
        let seg_height = grid.height / num_y;

        let mut segments: Vec<GridSegment> = vec![];

        for x in 0..num_x {
            for y in 0..num_y {
                segments.push( GridSegment::new(Point::new((seg_width* x) as f32, (seg_height * y) as f32), seg_width, seg_height));
            }
        }

        for seg in &mut segments {
            (seg.state, seg.id_list) = GridSegment::calc_state(&grid.reflex_verts, seg );
            
        }
        
        return segments;
        
    }
    
}

impl<'a> GridSegment {
    pub fn new(ankor: Point, width: u16, height: u16) -> GridSegment{
        GridSegment {
            ankor,
            width,
            height,
            state: SegState::None,
            id_list: vec![],
        }
    }

    //calculates the state of a grid segment freshly
    pub fn calc_state(reflex_verts: &[Vertex], segment: &mut GridSegment,) -> (SegState, Vec<u16>) {

        
        let mut ret_state: SegState = SegState::None;
        let mut ret_list: Vec<u16> = vec![];

        for i in 0..reflex_verts.len(){
           
            if reflex_verts[i].x >= segment.ankor.x && reflex_verts[i].x <= segment.ankor.x + (segment.width) as f32 {
                if reflex_verts[i].y >= segment.ankor.y && reflex_verts[i].y <= segment.ankor.y + (segment.height) as f32 {

                    ret_list.push(reflex_verts[i].id);
                    ret_state =  SegState::Reflex;
                    
                }
            }
        }
        return (ret_state, ret_list)
    }

    //updates the state of a grid segment if a vertex changed from reflex to convex
    pub fn update_state(seg: &mut GridSegment, id: u16) -> (Vec<u16>, SegState){
         
            let mut list = seg.id_list.clone();

            let mut i: usize = 0;

            while  i < seg.id_list.len() {
                if seg.id_list[i] == id {  list.remove(i);}
                i += 1;
            }
            
            if list.len() == 0 { return (list, SegState::None)} else {return (list, SegState::Reflex)}
            
        
        
        
    }

    

}

impl Stats {
    pub fn new() -> Stats{
        Stats { min_angles: vec![], areas: vec![] }
    }

    pub fn new_from_values(angles: Vec<f32>, areas: Vec<f32>) -> Stats {
        Stats { min_angles: angles, areas, }
    }
}

//calculates the signum of a vertex
pub fn get_signum_of_det(vertex: Vertex, v_l: Vertex, v_r: Vertex) -> char {

    let vec1_x = v_l.x - vertex.x;
    let vec1_y = v_l.y - vertex.y;
  
    let vec2_x = v_r.x - vertex.x;
    let vec2_y = v_r.y - vertex.y;
   
    let det = vec1_x * vec2_y - vec1_y * vec2_x;
    

    if det > 0.0 {  return '+' } else {return '-'}
}

//adds all reflex vertices to reflex_buffer
pub fn get_reflex_vertices(verts: &mut Vec<Vertex>) -> Vec<Vertex> {
    let mut reflex_buffer = vec![];

    for i in 0..verts.len(){
        let v_l: Vertex;
        let v_r: Vertex;
        if i == 0 { v_l = verts[verts.len() - 1]} else {v_l = verts[i-1]}
        if i == verts.len() - 1 {v_r = verts[0]} else { v_r = verts[i+1]}  
        verts[i].signum = get_signum_of_det(verts[i], v_l, v_r); 
       
    }

    //counting the negativ and positiv signums of the determinats
    //the det is giving the direction of the angle between to vetors
    let mut pos_count:u32 = 0;
    let mut neg_count:u32 = 0;

    for s in &mut *verts {
        if s.signum == '+' { pos_count += 1}
        else { neg_count += 1}
    }

    //the majority of vertices in a polygon always is convex, so the minority is reflex
    if pos_count > neg_count { 
        for v in verts {
            if v.signum == '-' { reflex_buffer.push( Vertex::new_with_sig(Point::new(v.x, v.y), v.id, '-'))}
        }
    }
    else { 
        for v in verts {
        if v.signum == '+' { reflex_buffer.push(Vertex::new_with_sig(Point::new(v.x, v.y), v.id, '+'))}
        }
    }
    
    return reflex_buffer;
}


