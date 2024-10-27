
use raylib::prelude::*;

use std::cell::RefCell;
use crate::ball::Ball;

pub trait Point {
    fn pos(&self) -> Vector2;
}



#[derive(Copy, Clone)]
pub struct Quad {
    pub center: Vector2,
    pub half_size: f32,
}

// Quadrant Labels:
// +----+----+
// | 0  | 2  |
// |(00)|(10)|
// +----+----+
// | 1  | 3  |
// |(01)|(11)|
// +----+----+

impl Quad {

    pub fn overlap(&self, other: &Quad) -> bool{
        let dx = (self.center.x - other.center.x).abs();
        let dy = (self.center.y - other.center.y).abs();
        dx < (self.half_size + other.half_size) && 
        dy < (self.half_size + other.half_size)
    }

    pub fn contains(&self, p: Vector2) -> bool {
        let dx = self.center.x - p.x;
        let dy = self.center.y - p.y;
        let max_dist = f32::max(dx.abs(), dy.abs());
        max_dist < self.half_size 
    }


    pub fn find_quadrant(&self, p: Vector2) -> usize {
        ((self.center.x < p.x) as usize) << 1 | (self.center.y < p.y) as usize
    }

    fn get_quadrant(&self, i: usize) -> Quad {
        if i >3 {panic!("Accessing a quadrant out of scope")}

        let quater_size = self.half_size / 2.0;
        let dx = match (i & 0b10) == 2 {
            | true  => quater_size,
            | false => -quater_size
        };

        let dy = match (i & 0b1) == 1 {
            | true  => quater_size,
            | false => -quater_size
        };

        Quad {
            center: Vector2{
                x: self.center.x + dx,
                y: self.center.y + dy
            },
            half_size: quater_size
        }
    }

    pub fn split_quadrants(&self) -> [Quad; 4] {
        [0, 1, 2, 3].map(|i| self.get_quadrant(i))
    }
}


pub struct QTNode {
    quad: Quad,
    children: usize,                // index to start of childen in Quadtree.nodes, 
                                    // if 0 then no childen
    ball_indices: Vec<usize>,              
}

impl QTNode {
    pub fn new(quad: Quad) -> Self {
        QTNode {
            quad ,
            children: 0 ,
            ball_indices: Vec::new(),
        }
    }

    pub fn is_branch(&self) -> bool {
        self.children != 0
    }
    pub fn is_leaf(&self) -> bool {
        self.children == 0
    }
    pub fn is_empty(&self) -> bool {
        self.ball_indices.len() == 0
    }
}



// Quadtree
pub struct Quadtree {
    pub balls:    Vec<Ball>,      // We put the balls here and the indexes are held in the nodes.    
    nodes:        Vec<QTNode> ,   // We put the nodes beside each other in space.
    boundary:     Quad,         
    split_thresh: usize,          // Threshhold for when to split
    min_size:     f32,            // Minimum size of quad where subdividing makes no sense.
}

impl Quadtree {
    pub const ROOT: usize = 0;

    pub fn new(boundary: Quad, split_thresh: usize, min_size: f32) -> Self {
        Quadtree {
            balls: Vec::new(),
            nodes: Vec::new(),
            boundary,
            split_thresh,
            min_size,
        }

    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.nodes.push(QTNode::new(self.boundary)); // The root node
    }

    pub fn push(&mut self, b: Ball) {
        self.balls.push(b);
        self.insert(b.pos, self.balls.len()-1);
    }

    fn insert(&mut self, pos: Vector2, index: usize) {
        
        if !self.boundary.contains(pos) {
            return   
        }

        let mut node = Self::ROOT;

        while self.nodes[node].is_branch() {
            let quadrant = self.nodes[node].quad.find_quadrant(pos);
            node = self.nodes[node].children + quadrant;
        }

        self.nodes[node].ball_indices.push(index); // Add the index of the ball to the node 

        
        if self.nodes[node].ball_indices.len() > self.split_thresh {  
            self.subdivide(node);
        }
    }

    fn subdivide(&mut self, node: usize) {
        // Make nodes for children
        let new_children = self.nodes.len(); // We are adding them right after.
        let sub_quads = self.nodes[node].quad.split_quadrants();
        for q in sub_quads {
            self.nodes.push(QTNode::new(q));
        }
        

        let balls_to_reinsert = self.nodes[node].ball_indices.clone();

        // Insert point into children
        for b in balls_to_reinsert {

            let quadrant = self.nodes[node].quad.find_quadrant(self.balls[b].pos);
            self.nodes[new_children + quadrant].ball_indices.push(b);
        } 

        self.nodes[node].ball_indices.clear();
        self.nodes[node].children = new_children;
    }

    pub fn rebuild(&mut self) {

        self.clear();

        let positions_and_indices: Vec<_> = self.balls.iter().enumerate().map(|(i, b)| (b.pos, i)).collect();
        
        for (pos, i) in positions_and_indices {
            self.insert(pos, i);
        }
    }

    pub fn query(&self, range: Quad) -> Vec<usize> {
        let mut result = Vec::new();
        self._query(&mut result, range, 0);
        result
    }

    fn _query(&self, result: &mut Vec<usize>, range: Quad, node_idx: usize) {
        let node = &self.nodes[node_idx];        

        if !node.quad.overlap(&range) {
            return
        }

        if node.is_leaf() {
            for b in &node.ball_indices {
                if range.contains(self.balls[*b].pos) {
                    result.push(*b);
                }
            }
        } else {
            for child in node.children..node.children+4 {
                self._query(result, range, child);
            }
        }
    }    
}

// -------  App specific stuff   --------- 

impl Quadtree {
    pub fn draw_tree(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        for node in &self.nodes {
            let hs = node.quad.half_size;
            let top_left = node.quad.center - Vector2{x: hs,y: hs};
            d.draw_rectangle_lines(top_left.x as i32, top_left.y as i32, (hs*2.0) as i32, (hs*2.0) as i32, Color::WHITE);
        }

    }

    pub fn debug(&self) {
       println!("Quadtree Nodes: {}",  self.nodes.len())
    }

    pub fn total_nodes (&self) -> usize {
        return self.nodes.len();
    } 

    pub fn total_balls(&self) -> usize {
        return self.balls.len();
    }
}
