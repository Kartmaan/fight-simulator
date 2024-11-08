//! Structures and methods for geometric operations in 2D space

/// 2D coordinates structure
#[derive(Debug, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    /// Create a new Pos struct
    pub fn new(x: i32, y:i32) -> Pos {
        Pos {x:x, y:y}
    }

    /// Initializes default coordinates
    pub fn default() -> Pos {
        Pos {x:0, y:0}
    }

    /// Change the coordinates of a Pos struct
    pub fn move_to(&mut self, x:i32, y:i32) {
        self.x = x;
        self.y = y;
    }

    /// Euclidian distance between two coordinates
    pub fn dist(&self, other:&Pos) -> f32 {
        let res = (other.x - self.x).pow(2) + (other.y - self.y).pow(2);
        (res as f32).sqrt()
    }
}