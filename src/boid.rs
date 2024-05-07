use nannou::prelude::*; 

#[derive(Clone)] 
#[derive(PartialEq)]
pub struct Boid{
    pub pos:Vec2,
    pub vel:Vec2,

}
//https://github.com/Programmazione-per-la-Fisica/progetto2022/blob/main/boids.md
impl Boid{
    pub fn new(pos:Vec2)->Self{
        Self{
            pos,
            vel: Vec2::new(random_range(-2., 2.), random_range(-2., 2.)), // Imposta il valore di default per 'vel' a zero
        }
    }
}