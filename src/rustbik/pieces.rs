use crate::rustbik::atomics::*;


#[derive(Debug, Copy, Clone)]
pub struct Piece {
    colors: [Color; FACES_N],
}

impl Piece {
    // Creates a new piece without any colors
    pub fn new() -> Piece { Piece{colors: [Color::None, Color::None, Color::None, Color::None, Color::None, Color::None]} }

    // Gets or sets the color of a face
    pub fn get_color(&self, f: Face) -> &Color { &self.colors[f as usize] }
    pub fn set_color(&mut self, f: Face, c: Color) { self.colors[f as usize] = c; }

    // Rotates the piece
    pub fn rotate(&mut self, a: Axis, prime: bool, double: bool) {
        // Calculates the cycle
        let mut cycle: [Face; 4] = match a {
            Axis::X => [Face::D, Face::B, Face::U, Face::F],
            Axis::Y => [Face::L, Face::F, Face::R, Face::B],
            Axis::Z => [Face::R, Face::U, Face::L, Face::D],
        };

        if double {
            let a = cycle[0] as usize;
            let b = cycle[1] as usize;
            let c = cycle[2] as usize;
            let d = cycle[3] as usize;

            // Swaps two pairs of opposites
            (self.colors[a], self.colors[b]) = (self.colors[b], self.colors[a]);
            (self.colors[c], self.colors[d]) = (self.colors[d], self.colors[c]);
        } else {
            // If it's prime reverses the cycle
            if prime { cycle.reverse(); }

            // Swaps the colors
            let backup = self.colors[cycle[0] as usize];
            for i in 0..3 {
                self.colors[cycle[i % 4] as usize] = self.colors[cycle[(i + 1) % 4] as usize];
            }
            self.colors[cycle[3] as usize] = backup;
        }
    }
}
