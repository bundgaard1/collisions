
use crate::constants::*;
use crate::BallCollection;
use crate::quadtree::*;
use crate::ball::Ball;

use raylib::math::Vector2;
use raylib::prelude::*;


pub struct QuadtreeBallCollection {
    quadtree: Quadtree<Ball>,
}

impl QuadtreeBallCollection {

}

impl BallCollection for QuadtreeBallCollection {
    
    fn new() -> Self {
        let screen_quad = Quad {
            center: Vector2 {x: WINDOW_WIDTH as f32 / 2.0,y: WINDOW_HEIGHT as f32 / 2.0},
            half_size: WINDOW_HEIGHT as f32 / 2.0
        };
    
        Self {
            quadtree: Quadtree::new(screen_quad, 4, BALL_RADIUS*2.0)
        }
    }

    fn create_balls(&mut self, count: usize) { 
        self.quadtree.clear();
        for _ in 0..count {
            let b = Ball::new_random();
            self.quadtree.push(b);

        }
    }

    fn update(&mut self, delta: f32) { 
        for b in &mut self.quadtree.points {
            b.update(delta);
        }

        // Rebuild the quadtree, based on the new positions
        self.quadtree.rebuild();
        
        for i in 0..self.quadtree.points.len() {
            let b_pos = self.quadtree.points[i].pos();
            let potential_collisions = self.quadtree.query(Quad { center: b_pos, half_size: BALL_RADIUS * 2.0 });
        
            for &j in &potential_collisions {
                if i != j {
                    let b = unsafe { &mut *(&mut self.quadtree.points[i] as *mut Ball) };
                    let b2 = unsafe { &mut *(&mut self.quadtree.points[j] as *mut Ball) };
        
                    if b.overlap(b2) {
                        b.resolve_collision(b2);
                    }
                }
            }
        }

        
    }

    

    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle) {
        for b in &self.quadtree.points {
            d.draw_circle(b.pos.x as i32, b.pos.y as i32, BALL_RADIUS as f32, Color::YELLOW);
        }
        self.quadtree.draw_tree(d);
        
        // Draw a range around the mouse
        let mouse = d.get_mouse_position();

        let range = Quad {
            center: mouse,
            half_size: 50.0,
        };
        self.quadtree.query(range).iter().for_each(|&i| {
            let b = &self.quadtree.points[i];
            d.draw_circle(b.pos.x as i32, b.pos.y as i32, BALL_RADIUS as f32, Color::RED);
        });

        d.draw_rectangle_lines(range.center.x as i32 - range.half_size as i32, range.center.y as i32 - range.half_size as i32, (range.half_size*2.0) as i32, (range.half_size*2.0) as i32, Color::RED);
 
    }
}