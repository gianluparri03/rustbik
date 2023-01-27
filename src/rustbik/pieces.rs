use crate::rustbik::atomics::*;


#[derive(Default, Debug, Copy, Clone)]
pub struct Piece {
    u: Option<Color>,
    d: Option<Color>,
    f: Option<Color>,
    b: Option<Color>,
    l: Option<Color>,
    r: Option<Color>,
}

impl Piece {
    // Gets the color of a face as a mutable reference
    fn get_color_mut(&mut self, f: Face) -> &mut Option<Color> {
        match f {
            Face::U => &mut self.u,
            Face::D => &mut self.d,
            Face::F => &mut self.f,
            Face::B => &mut self.b,
            Face::L => &mut self.l,
            Face::R => &mut self.r,
        }
    }

    // Gets the color of a face as a reference
    pub fn get_color(&self, f: Face) -> &Option<Color> {
        match f {
            Face::U => &self.u,
            Face::D => &self.d,
            Face::F => &self.f,
            Face::B => &self.b,
            Face::L => &self.l,
            Face::R => &self.r,
        }
    }

    // Puts a color in a face
    fn set_color(&mut self, f: Face, c: Color) {
        *self.get_color_mut(f) = Some(c);
    }
}


// Creates a void piece (no colors)
pub fn new_void() -> Piece {
    Piece{..Default::default()}
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
