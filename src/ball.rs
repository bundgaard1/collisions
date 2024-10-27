use crate::constants::*;

use rand::Rng;
use raylib::prelude::*;
use crate::quadtree::Point;

#[derive(Copy, Clone, PartialEq)]
pub struct Ball  {
    pub pos: Vector2,
    pub vel: Vector2,
}

impl Point for Ball {
    fn pos(&self) -> Vector2 {
        self.pos
    }
}

impl Ball {

    pub fn new(pos: Vector2, vel: Vector2) -> Self {
        Ball {pos,vel}
    }

    pub fn new_random() -> Self {

        let mut rng = rand::thread_rng();
        Ball {
            pos: Vector2 {x: rng.gen_range(50.0..(WINDOW_WIDTH-50) as f32), y: rng.gen_range(50.0..(WINDOW_HEIGHT-50) as f32) },
            vel: Vector2::one().rotated(rng.gen_range(0.0..360.0)).scale_by(INITIAL_SPEED),
        }
    }


    pub fn update(&mut self, delta: f32) {
        
        // Position update
        self.pos = self.pos + self.vel * delta;

        // Edge detection
        if self.pos.x - BALL_RADIUS < 0.0 {
            self.vel.x = self.vel.x.abs(); // Make velocity positive
        } else if (WINDOW_WIDTH as f32) < self.pos.x + BALL_RADIUS {
            self.vel.x = -self.vel.x.abs(); // Make velocity negative
        }

        if self.pos.y - BALL_RADIUS < 0.0 {
            self.vel.y = self.vel.y.abs(); // Make velocity positive
        } else if (WINDOW_HEIGHT as f32) < self.pos.y + BALL_RADIUS {
            self.vel.y = -self.vel.y.abs(); // Make velocity negative
        }
    }

    pub fn overlap(&self, other : &Ball ) -> bool {
        let diff = self.pos.distance_to(other.pos);
        diff < 2.0 * BALL_RADIUS
    }

    pub fn resolve_collision(&mut self, other: &mut Ball) {

        let mut impact = other.pos - self.pos;
        let mut d = impact.length();

        // Push balls away from each other
        let overlap = d - 2.0 * BALL_RADIUS;
        if overlap > 0.0 { // Extra check, to not fuck it all up lol
            return;
        }

        let offset = impact.normalized().scale_by(overlap / 2.0);
        self.pos = self.pos + offset;
        other.pos = other.pos - offset;
        
        // Correct distance
        d = 2.0 * BALL_RADIUS;
        impact.normalize();
        impact.scale(d);

        let m = BALL_MASS;
        let mass_sum = 2.0 * m;
        let vel_diff = other.vel - self.vel;

        let num = vel_diff.dot(impact);
        let den = mass_sum * d * d;

        // Update velocity of self
        let mut delta_vel_self = impact.clone();
        delta_vel_self.scale((2.0 * m * num) / den);
        self.vel += delta_vel_self;

        // Update velocity of other
        let mut delta_vel_other = impact.clone();
        delta_vel_other.scale((-2.0 * m * num) / den);
        other.vel += delta_vel_other;

    }
    
}