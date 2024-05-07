//www.kfish.org/boids/pseudocode.html
extern crate nannou;
mod boid;
mod header;
use boid::*;
use header::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

///The Model is where we define the state of our application.
///  We can think of the model as the representation of our program at any point in time. 
/// Throughout the life of our program, we can update the model as certain events occur such as mouse presses, key presses or timed updates. 
/// We can then present the model using some kind of output, e.g. by drawing to the screen or outputting to a laser. 
/// We will look at these input and output events in more detail in another tutorial! Our example is as simple as possible, and we have no state to track. 
/// Thus our model can stay empty.
struct Model {
    _window: window::Id,
    flock: Vec<Boid>,
}

///The model function is run once at the beginning of the nannou app and produces a fresh, 
/// new instance of the Model that we declared previously, AKA the app state. 
/// This can be thought of as the "setup" stage of our application. Here, we might do things like create some windows, create a GUI, 
/// load some images or anything else that we want to happen once at the beginning of our program. 
/// We will learn how to do all of these things in future tutorials, but for now we will just return an instance of our empty Model.
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
fn model(app: &App) -> Model {


    let width_f32 = WIDTH as f32;
    let height_f32 = HEIGHT as f32;
    let _window = app.new_window().size(WIDTH, HEIGHT).view(view).build().unwrap();

    let mut flock: Vec<Boid>= vec![];
    for _i in 0..200{
        flock.push(Boid::new(vec2(random_range(-width_f32/2., width_f32/2.), random_range(-height_f32/2., height_f32/2.0))));
    }

    Model {
        _window,
        flock,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //cohesion(&model.flock);
    move_boids(&mut model.flock, &WIDTH, &HEIGHT);
}

///Finally, the view allows us to present the state of the model to a window by drawing to its Frame and returning the frame at the end. 
/// Here we can change the background colour, use the Draw API to draw a scene, 
/// draw a GUI to the window or even use the wgpu API to draw to the frame using 
/// our own textures and render passes. All of this will be covered by future tutorials.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for point in model.flock.iter() {
        draw.ellipse()
            .color(WHITE)
            .radius(5.0)
            .xy(point.pos);
    }
    draw.to_frame(app, &frame).unwrap();
}

