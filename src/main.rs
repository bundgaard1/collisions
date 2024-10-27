mod ball;
mod ball_collection;
mod ball_collection_qt;
mod constants;
mod quadtree;

use ball_collection::*;
use ball_collection_qt::QuadtreeBallCollection;
use constants::*;
use raylib::prelude::*;

fn main() {
    demo::<SimpleBallCollection>();
    demo::<QuadtreeBallCollection>();
}

fn demo<BC: BallCollection>() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, World")
        .build();

    let mut ball_collection = BC::new();

    ball_collection.create_balls(N_BALLS as usize);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        // Balls update
        ball_collection.update(d.get_frame_time());
        ball_collection.draw(&mut d);

        // FPS
        d.draw_text(
            &format!("simulation time: {} ms", d.get_frame_time() * 1000.0).as_str(),
            12,
            12,
            18,
            Color::WHITE,
        );
        d.draw_fps(12, 32);
    }
}
