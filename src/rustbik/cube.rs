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
                c.pieces[0][i][j].set_color(Face::U, Some(Color::Yellow));
                c.pieces[2][i][j].set_color(Face::D, Some(Color::White));
                c.pieces[i][0][j].set_color(Face::B, Some(Color::Red));
                c.pieces[i][2][j].set_color(Face::F, Some(Color::Orange));
                c.pieces[i][j][0].set_color(Face::L, Some(Color::Green));
                c.pieces[i][j][2].set_color(Face::R, Some(Color::Blue));
            }
        }

        c
    }

    pub fn get_face(&self, f: Face) -> PiecesGroup {
        let mut pg = PiecesGroup::new(vec![]);

        // Adds all the pieces of that face to the group in order
        for i in 0..3 {
            for j in 0..3 {
                pg.push(match f {
                    Face::U => &self.pieces[0][i][j],
                    Face::D => &self.pieces[2][i][2-j],
                    Face::B => &self.pieces[i][0][2-j],
                    Face::F => &self.pieces[i][2][j],
                    Face::L => &self.pieces[i][j][0],
                    Face::R => &self.pieces[i][2-j][2],
                });
            }
        }

        pg
    }
}
