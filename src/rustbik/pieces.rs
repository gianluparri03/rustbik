use crate::rustbik::atomics::*;


#[derive(Default, Copy, Clone)]
pub struct Piece {
    up: Option<Color>,
    down: Option<Color>,
    front: Option<Color>,
    back: Option<Color>,
    left: Option<Color>,
    right: Option<Color>,
}

impl Piece {
    // Puts a color in a face
    fn set_color(&mut self, f: Face, c: Color) {
        match f {
            Face::Up => { self.up = Some(c); },
            Face::Down => { self.down = Some(c); },
            Face::Front => { self.front = Some(c); },
            Face::Back => { self.back = Some(c); },
            Face::Left => { self.left = Some(c); },
            Face::Right => { self.right = Some(c); },
        }
    }
}


// Creates a center (only a color)
pub fn new_center(f1: Face, c1: Color) -> Piece {
    let mut p = Piece{..Default::default()};
    p.set_color(f1, c1);
    p
}

// Creates an edge (two colors)
pub fn new_edge(f1: Face, c1: Color, f2: Face, c2: Color) -> Piece {
    let mut p = Piece{..Default::default()};
    // TODO: ensure faces are adjacent
    p.set_color(f1, c1);
    p.set_color(f2, c2);
    p
}

// Creates a corner (three colors)
pub fn new_corner(f1: Face, c1: Color, f2: Face, c2: Color, f3: Face, c3: Color) -> Piece {
    let mut p = Piece{..Default::default()};
    // TODO: ensure faces are adjacent
    p.set_color(f1, c1);
    p.set_color(f2, c2);
    p.set_color(f3, c3);
    p
}
