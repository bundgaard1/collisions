
use crate::constants;
use crate::constants::*;
use crate::BallCollection;
use crate::quadtree::*;
use crate::ball::Ball;

use raylib::math::Vector2;
use raylib::prelude::*;


pub struct QuadtreeBallCollection {
    balls: Vec<Ball>,
    quadtree: Quadtree,
}

impl QuadtreeBallCollection {
    pub fn new() -> Self {
        let screen_quad = Quad {
            center: Vector2 {x: WINDOW_WIDTH as f32 / 2.0,y: WINDOW_HEIGHT as f32 / 2.0},
            half_size: WINDOW_HEIGHT as f32 / 2.0
        };

        Self {
            balls: Vec::new(),
            quadtree: Quadtree::new(screen_quad, 8, BALL_RADIUS*2.0)
        }
    }

    pub fn debug(&self) {
        self.quadtree.debug();
    }
}

impl BallCollection for QuadtreeBallCollection {

    fn create_balls(&mut self, count: usize) { 
        for _i in 0..count {
            let b = Ball::new_random();
            self.balls.push(b);  
        }
    }

    fn update(&mut self, delta: f32) {
        self.quadtree.clear();
        let mut nearby_points: Vec<usize> = Vec::new();

        for i in 0..self.balls.len() {
            self.balls[i].update(delta);
            let b = &self.balls[i];
            self.quadtree.insert((b.pos, i));
        }

        // Second pass: resolve collisions
        for i in 0..self.balls.len() {
            let b_pos = self.balls[i].pos;
            self.quadtree.query_range(&mut nearby_points, Quad { center: b_pos, half_size: constants::BALL_RADIUS * 1.5 });

            for &j in &nearby_points {
                if i != j {
                    let (left, right) = self.balls.split_at_mut(j);
                    if i < j {
                        left[i].resolve_collision(&mut right[0]);
                    }
                }
            }

            nearby_points.clear();
        }
       
        println!("Total nodes: {}",self.quadtree.total_nodes())
    }

    

    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        for b in &self.balls{
            d.draw_circle(b.pos.x as i32, b.pos.y as i32, BALL_RADIUS, Color::ROYALBLUE);
        } 
        // self.quadtree.draw_tree(d);
    }
}