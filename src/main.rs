mod ball;
mod constants;
mod ball_collection;
mod ball_collection_qt;
mod quadtree;

use constants::*;
use ball_collection::*;
use ball_collection_qt::QuadtreeBallCollection;
use raylib::prelude::*;

fn main() {

    demo();
}


fn demo() {
    // let mut balls = SimpleBallCollection::new();
    let mut balls = QuadtreeBallCollection::new();

    balls.create_balls(N_BALLS as usize);
    

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, World")
        .build();
    

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);

        // Balls update
        balls.update(d.get_frame_time());
        balls.draw(&mut d);

        // FPS
        d.draw_text(&format!("simulation time: {} ms", d.get_frame_time() * 1000.0).as_str(), 12, 12, 18, Color::WHITE);
        d.draw_fps(12, 32);

        // Debug
    }
}