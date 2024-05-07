use std::vec;
use nannou::prelude::*;
use crate::boid::*;
const VISUAL_RANGE : f32 = 130.;
const SEPARATION_RANGE : f32 = 40.;
const V_LIM : f32 = 10.;
const COHESION_PERCENT : f32 = 20.;
const SEPARATION_PERCENT : f32 = 40.;


pub fn move_boids(flocks: &mut Vec<Boid>, width: &u32 , height:&u32) {
    let mut v1: Vec2;
    let mut v2: Vec2;
    let mut v3: Vec2;
    let mut v4: Vec2;
    let flocks_copy: Vec<Boid> = flocks.clone();
    for b in &mut *flocks {
        v1 = cohesion(&flocks_copy, b);
        v2 = separation(&flocks_copy, b);
        v3 = alignment(&flocks_copy, b);
        v4 = bound_position(b, *width, *height);
        b.vel = b.vel +v1+ v2 +v3 + v4;
        limit_vel(b);
        b.pos += b.vel;
    }
}

fn cohesion(flocks:&Vec<Boid>, boid : &Boid) -> Vec2 {
    let mut nears: Vec<Vec2> = vec![];
    //finding in view area
    for current in flocks {
        if boid != current {
            if boid.pos.distance(current.pos) < VISUAL_RANGE {
                nears.push(current.pos);
            }
        }
    }
    println!("nears frjh3r:{:?}", nears);
    if nears.is_empty() {
        return boid.vel; // return a default value when there are no near boids
    }
    let mut sum : Vec2 = vec2(0.,0.); 
    for vec in &nears{
        sum += *vec;
    }
    //finding average 
    let avg:Vec2;
    if nears.len() > 0 {
        avg = sum / (nears.len() as f32);
    } else {
        avg = Vec2::new(0.0, 0.0); 
    }

    return (avg-boid.pos) * (COHESION_PERCENT/100.);
}

fn separation(flocks:&Vec<Boid>, boid : &Boid)-> Vec2{
    let mut separation: Vec2 = vec2(0., 0.);
    for b in flocks{
        if boid != b{
            if abs(boid.pos.distance(b.pos)) < SEPARATION_RANGE{
                separation = separation - (b.pos - boid.pos); 
            }
        }
    }
    //println!("{} {}" , separation, separation*SEPARATION_PERCENT);
    return separation * (SEPARATION_PERCENT)/100.;
}


fn alignment(flocks:&Vec<Boid>, boid : &Boid)-> Vec2{
    let mut nears: Vec<Vec2> = vec![];
    //finding in view area
    for current in flocks {
        if boid != current {
            if boid.pos.distance(current.pos) < VISUAL_RANGE {
                nears.push(current.vel);
            }
        }
    }

    let mut sum : Vec2 = vec2(0.,0.); 
    for vec in &nears{
        sum += *vec;
    }

    //finding average 
    let avg:Vec2;
    if nears.len() > 0 {
        avg = sum / (nears.len() as f32);
        return avg - boid.vel;
    } else {
        return Vec2::new(0.0, 0.0); 
    }
}

fn limit_vel(boid:&mut Boid){
    let magnitude = abs(boid.vel.x.powi(2) + boid.vel.y.powi(2)).sqrt();
    if magnitude > V_LIM {
        boid.vel = boid.vel / magnitude * V_LIM;
    }
}

pub fn bound_position(b: &Boid, width: u32, height: u32) -> Vec2 {
    let mut v = Vec2::new(0.0, 0.0);
    const TURN: f32 = 1.;

    if b.pos.x < -(width as f32 / 2.0) {
        v.x = TURN;
    } else if b.pos.x > (width as f32 / 2.0) {
        v.x = -TURN;
    }

    if b.pos.y < -(height as f32 / 2.0) {
        v.y = TURN;
    } else if b.pos.y > (height as f32 / 2.0) {
        v.y = -TURN;
    }

    return v
}



