use std::f32::consts::PI;

use iced::Point;
use crate::dep::modules::geometry::Vertex;

use super::super::modules::geometry::{Line, BoundingBox};
use super::grid::{Grid, SegState, get_signum_of_det, GridSegment, Stats};

pub fn perform_primitiv_eca_step( grid: &mut Grid, diagonal_buffer: &mut Vec<Line>) -> (Vec<Vertex>,Vec<Vertex>,Vec<Line>, (f32, f32)){

    let vertices =  &mut grid.vertices;
    let reflex_vertices =  &mut grid.reflex_verts;
    let mut num_of_verts = vertices.len();
    let mut stat_tupel= (0.0, 0.0);
    //get first convex vertex and try Ear Clipping, if not possible go on with next convex vertex
    for i in 0..num_of_verts -1{
       
        if !check_if_reflex(vertices[i], &reflex_vertices) {
            
            //getting the vertices for the EAR to cut
            let v = vertices[i];
            let v_l: Vertex;
            let v_r: Vertex;
            if i == 0 { v_l = vertices[num_of_verts - 1]} else {v_l = vertices[i-1]}
            if i == vertices.len() - 1 {v_r = vertices[0]} else { v_r = vertices[i+1]} 

            let bbox = calulate_bunding_box(v, v_l, v_r);
            if !check_grid_for_reflex_state(bbox, &mut grid.grid_segments, v_l.id, v_r.id) {
                vertices.remove(i);
                
                diagonal_buffer.push(Line::new(Point::new(v_l.x, v_l.y), Point::new(v_r.x, v_r.y)));

                stat_tupel = get_stats(v, v_l, v_r);
                

                num_of_verts -= 1;

                //check if v_l changed from reflex to convex and update sgements if needed
                if check_if_reflex(v_l, &reflex_vertices) {
                    let old_sig = v_l.signum;
                    
                    let mut u: Vertex = v_l; //left of v_l, assigning just for compiler error resolving
                    
                    for i in 0..vertices.len() - 1 {
                        if vertices[i].id == v_l.id {
                            if i == 0 { u = vertices[num_of_verts - 1]; break;} else {u = vertices[i-1]; break;}
                        }
                        
                    }

                    if old_sig != get_signum_of_det(v_l, u, v_r) {
                        for j in 0..reflex_vertices.len() -1 {
                            if reflex_vertices[j].id == v_l.id { 
                                reflex_vertices.remove(j);
                                
                                for seg in &mut grid.grid_segments {
                                    if seg.state == SegState::Reflex {(seg.id_list, seg.state) = GridSegment::update_state( seg, v_l.id);}
                                }

                               
                            }
                        }
                    }

                    
                }

                //check if v_r changed from reflex to convex and update sgements if needed
                if check_if_reflex(v_r, &reflex_vertices) {
                    let old_sig = v_r.signum;
                    
                    let mut u: Vertex = v_r; //right of v_r, assigning just for compiler error resolving
                    
                    for i in 0..vertices.len() - 1 {
                        if vertices[i].id == v_r.id {
                            if i == vertices.len() - 1 { u = vertices[0]; break;} else {u = vertices[i+1]; break;}
                        }
                        
                    }

                    if old_sig != get_signum_of_det(v_r, v_l, u) {
                        for j in 0..reflex_vertices.len() -1 {
                            if reflex_vertices[j].id == v_r.id { 
                                reflex_vertices.remove(j);
                                for seg in &mut grid.grid_segments {
                                    if seg.state == SegState::Reflex {(seg.id_list, seg.state) = GridSegment::update_state( seg, v_r.id);}
                                    
                                }
                            }
                        }
                    }

                    
                }
                
                break;
            }
            
        }   
    }

    return (vertices.to_vec(), reflex_vertices.to_vec(), diagonal_buffer.to_vec(), stat_tupel );
}

fn check_if_reflex(vertex: Vertex, reflex_vertices: &[Vertex]) -> bool {
    for v in reflex_vertices {
        if v.id == vertex.id { return true} 
    }
    return false;
}

fn calulate_bunding_box(v: Vertex, v_l: Vertex, v_r: Vertex) -> BoundingBox {

    let mut min_x: f32 = v.x;
    let mut min_y: f32 = v.y;
    let mut max_x: f32 = v.x;
    let mut max_y: f32 = v.y;
  
    for vertex in [v_l, v_r] {
        if vertex.x < min_x { min_x = vertex.x}
        if vertex.x > max_x { max_x = vertex.x}
        if vertex.y < min_y { min_y = vertex.y}
        if vertex.y > max_y { max_y = vertex.y}
    }
    return BoundingBox::new(Point::new(min_x, min_y), max_x - min_x, max_y - min_y)
}

fn check_grid_for_reflex_state(bbox: BoundingBox, segments: &mut Vec<GridSegment>, v_l_id: u16, v_r_id: u16) -> bool {

    //get all GridSegments in the BoundingBox
    for seg in segments {
        if seg.state == SegState::Reflex {
        
            for i in 0..20 {
                let frac_x = if i == 0 { 0.0} else {1.0/i as f32};
                let seg_point_x = seg.ankor.x + seg.width as f32 * frac_x;
                if seg_point_x >= bbox.ankor.x && seg_point_x <= bbox.ankor.x + bbox.width {
                
                    for j in 0..20 {
                        let frac_y = if i == 0 {0.0 } else {1.0 / j as f32};
                        let seg_point_y = seg.ankor.y + seg.height as f32* frac_y;
                        if seg_point_y >= bbox.ankor.y && seg_point_y <= bbox.ankor.y + bbox.height {
                          
                              for id in &seg.id_list {
                    
                                if *id != v_l_id && *id != v_r_id { return true }
                                
                            }
                            break;
                        }
                    }
                    
                }
            }
            
        }
    }
    
    return false;
}

//funtcion that returns min angle of the triangle and its area
fn get_stats(v: Vertex, u: Vertex, w: Vertex) -> (f32, f32){

    //calculating vectors of trinagle
    let vu = [v.x - u.x, v.y - u.y];
    let vw = [v.x - w.x, v.y - w.y];
    let uw = [u.x - w.x, u.y - w.y];

    return (get_min_angle(vu, vw, uw), get_area(vu, vw))
}

fn get_min_angle(vec1: [f32; 2], vec2: [f32; 2], vec3: [f32; 2]) -> f32{

    let len_1 = (vec1[0]* vec1[0] + vec1[1] * vec1[1]).sqrt();
    let len_2 = (vec2[0]* vec2[0] + vec2[1] * vec2[1]).sqrt();
    let len_3 = (vec3[0]* vec3[0] + vec3[1] * vec3[1]).sqrt();
    //calculation angle between 2 of the 3 vectors
    let a = ((vec1[0] * vec2[0] + vec1[1] * vec2[1]) / (len_1 * len_2)).acos() * 180.0/PI;
    let b = ((vec1[0] * vec3[0] + vec1[1] * vec3[1]) / (len_1 * len_3)).acos() * 180.0/PI;
    let c = ((vec3[0] * vec2[0] + vec3[1] * vec2[1]) / (len_3 * len_2)).acos() * 180.0/PI;

    let mut out = a;
    if b < out {
        out = b;
    }
    if c < out {
        out = c;
    }
    return out
}

fn get_area(vec1: [f32; 2], vec2: [f32; 2]) -> f32{

    let len_1 = (vec1[0]* vec1[0] + vec1[1] * vec1[1]).sqrt();
    let len_2 = (vec2[0]* vec2[0] + vec2[1] * vec2[1]).sqrt();
    let a = ((vec1[0] * vec2[0] + vec1[1] * vec2[1]) / (len_1 * len_2)).acos() * 180.0/PI;

    let area = 0.5 * len_1 * len_2 * a.sin();
   
    return area.abs()
}

