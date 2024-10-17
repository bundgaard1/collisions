
use crate::ball::Ball;
use crate::constants::*;
use raylib::prelude::*;



pub trait BallCollection {
    fn create_balls(&mut self, count: usize);
    fn update(&mut self, delta: f32);
    fn draw(&self, d: &mut RaylibDrawHandle);
}

pub struct SimpleBallCollection  {
    balls : Vec<Ball>
}

impl SimpleBallCollection {
    pub fn new() -> Self {
        SimpleBallCollection {
            balls: Vec::new()
        }
    }

    pub fn total_enery(&self) -> f32 {
        let mut total: f32 = 0.0;
        for c in &self.balls {
            total += c.vel.length_sqr() * BALL_MASS; 
        }
        total
    }

}

impl BallCollection for SimpleBallCollection {
    
    fn create_balls(&mut self, count: usize) { 
        for _i in 0..count {
            let b = Ball::new_random();
            self.balls.push(b);  
        }
    }

    fn update(&mut self, delta : f32 ) {
        for i in 0..self.balls.len() {
    
            let (left, right) = self.balls.split_at_mut(i+1);
            let b = &mut left[i];
    
            b.update(delta);
            
            // Overlap & collisions
            for b2 in right {
                if b.overlap(b2) {
                    b.resolve_collision(b2);
                }
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        for b in &self.balls{
            d.draw_circle(b.pos.x as i32, b.pos.y as i32, BALL_RADIUS, Color::ROYALBLUE);
        } 
    }
}

