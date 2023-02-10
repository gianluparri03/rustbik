use crate::rustbik::atomics::*;
use crate::rustbik::pieces::*;


pub struct Cube {
    pieces: [[[Piece; 3]; 3]; 3]
}

impl Cube {
    // Creates a new cube
    pub fn new() -> Cube {
        let mut c = Cube{pieces: [[[Piece::new(); 3]; 3]; 3]};

        // Colors the faces
        for i in 0..3 {
            for j in 0..3 {
                c.pieces[0][i][j].set_color(Face::U, Color::Yellow);
                c.pieces[2][i][j].set_color(Face::D, Color::White);
                c.pieces[i][0][j].set_color(Face::B, Color::Red);
                c.pieces[i][2][j].set_color(Face::F, Color::Orange);
                c.pieces[i][j][0].set_color(Face::L, Color::Green);
                c.pieces[i][j][2].set_color(Face::R, Color::Blue);
            }
        }

        c
    }

    pub fn get_face(&self, f: Face) -> [&Color; 9] {
        let mut colors: [&Color; 9] = [&Color::None; 9];

        // Adds all the colors of that face to the group in order
        for i in 0..3 {
            for j in 0..3 {
                colors[i*3 + j] = match f {
                    Face::U => &self.pieces[0][i][j].get_color(f),
                    Face::D => &self.pieces[2][i][2-j].get_color(f),
                    Face::B => &self.pieces[i][0][2-j].get_color(f),
                    Face::F => &self.pieces[i][2][j].get_color(f),
                    Face::L => &self.pieces[i][j][0].get_color(f),
                    Face::R => &self.pieces[i][2-j][2].get_color(f),
                };
            }
        }

        colors
    }
}
