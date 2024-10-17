
use raylib::prelude::*;

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


struct QTNode {
    quad: Quad,
    children: usize,                // index to start of childen in Quadtree.nodes, 
                                    // if 0 then no childen
    balls: Vec<(Vector2, usize)>,   // pos, index into balls  
}

impl QTNode {
    pub fn new(quad: Quad) -> Self {
        QTNode {
            quad ,
            children: 0 ,
            balls: Vec::new(),
        }
    }

    pub fn is_branch(&self) -> bool {
        self.children != 0
    }
    pub fn is_leaf(&self) -> bool {
        self.children == 0
    }
    pub fn is_empty(&self) -> bool {
        self.balls.len() == 0
    }
}


// Quadtree
pub struct Quadtree {
    nodes:        Vec<QTNode> ,   // We put the nodes beside each other in space.
    boundary:     Quad,         
    split_thresh: usize,          // Threshhold for when to split
    min_size:     f32,            // Minimum size of quad where subdividing makes no sense.
}

impl Quadtree {
    pub const ROOT: usize = 0;

    pub fn new(boundary: Quad, split_thresh: usize, min_size: f32) -> Self {
        Quadtree {
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

    pub fn insert(&mut self, b: (Vector2, usize)) {
        // println!("Inserting: ({},{}), index: {}", b.0.x, b.0.y,b.1);
        
        if !self.boundary.contains(b.0) {
            return   
        }

        let mut node = Self::ROOT;

        while self.nodes[node].is_branch() {
            let quadrant = self.nodes[node].quad.find_quadrant(b.0);
            node = self.nodes[node].children + quadrant;
        }
        
        self.nodes[node].balls.push(b);

        
        if self.nodes[node].balls.len() > self.split_thresh {  
            self.subdivide(node);
        }
    }

    fn subdivide(&mut self, node: usize) {
        // Make nodes for children
        let new_children = self.nodes.len(); // We are adding them right after.
        let sub_quads = self.nodes[node].quad.split_quadrants();
        for q in sub_quads {
            // println!("Added new node: ({},{}),{}", q.center.x, q.center.y, q.half_size);
            self.nodes.push(QTNode::new(q));
        }

        // Insert point into children
        for i in 0..self.nodes[node].balls.len() {
            let b = self.nodes[node].balls[i];
            let quadrant = self.nodes[node].quad.find_quadrant(b.0);
            self.nodes[new_children + quadrant].balls.push(b);
        }  

        self.nodes[node].balls.clear();
        self.nodes[node].children = new_children;
    }

    pub fn query_range(&self,points_in_range: &mut Vec<usize>, range: Quad) {
        self._query_range(points_in_range, range, 0);
    }

    fn _query_range(&self, points_in_range: &mut Vec<usize>, range: Quad, node: usize) {
        
        if !self.nodes[node].quad.overlap(&range) {
            return
        }

        match self.nodes[node].children {
        | 0 => 
            for b in &self.nodes[node].balls {
                if range.contains(b.0) {
                    points_in_range.push(b.1);
                }
            },
        | children_index => 
            for child in children_index..children_index+4 {
                self._query_range(points_in_range, range, child);
            }  
        }
    } 

}


// App specific stuff

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
}
