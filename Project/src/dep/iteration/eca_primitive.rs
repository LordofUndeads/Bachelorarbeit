use iced::Point;

use crate::dep::modules::heuristic::Heuristic;

pub struct Diagonal {
    pub start_idx: u16,
    pub end_idx: u16
}

pub fn eca_primitive(vertices: Vec<Point>, diagonals: Vec<Diagonal>, clipped_vertices: Vec<Point>, heuristic: Heuristic) -> (Vec<Point>, Vec<Diagonal>, Vec<Point>){

    //get a tuple of v_i-1, v_i, v_i+1, wher v_i is the vertex to check the earness
    let mut nxt_idx: usize;
    let mut prv_idx: usize;
    for idx in 0..vertices.len() -1 {
        if idx == 0 { prv_idx = vertices.len() - 1} else {prv_idx = idx -1}
        if idx == vertices.len() - 1 {nxt_idx = 0} else {nxt_idx = idx +1}

        check_earness(vertices[prv_idx], vertices[idx], vertices[nxt_idx]);
    }
    

    //choose with heuristic




    return (vertices, diagonals, clipped_vertices);
}

fn check_earness(u: Point, v: Point, w: Point) -> bool {
   
    

    //calculate reflex vertices
    //these are candidates for ear tips

    //check if diagonal is internal

    //check if there is no interseption of the outline and the diagonal
    return false;
}