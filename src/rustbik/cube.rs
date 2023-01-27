use crate::rustbik::atomics::*;
use crate::rustbik::pieces::*;


pub struct Cube {
    pieces: [[[Piece; 3]; 3]; 3]
}

impl Cube {
    pub fn get_face(&self, f: Face) -> [&Option<Color>; 9] {
        let mut view: [&Option<Color>; 9] = [&None; 9];

        // Picks the right pieces in the right position
        for i in 0..3 {
            for j in 0..3 {
                match f {
                    Face::U => { view[i*3 + j] = self.pieces[0][i][j].get_color(f); },
                    Face::D => { view[i*3 + j] = self.pieces[2][i][2 - j].get_color(f); },
                    Face::B => { view[i*3 + j] = self.pieces[i][0][2 - j].get_color(f); },
                    Face::F => { view[i*3 + j] = self.pieces[i][2][j].get_color(f); },
                    Face::L => { view[i*3 + j] = self.pieces[i][j][0].get_color(f); },
                    Face::R => { view[i*3 + j] = self.pieces[i][2 - j][2].get_color(f); },
                }
            }
        }

        view
    }
}


// Creates a new, solved cube
pub fn new_cube() -> Cube {
    Cube{pieces: [
        [
            [
                new_corner(Face::U, Color::Yellow, Face::B, Color::Red, Face::L, Color::Green),
                new_edge(Face::U, Color::Yellow, Face::B, Color::Red),
                new_corner(Face::U, Color::Yellow, Face::B, Color::Red, Face::R, Color::Blue),
            ],
            [
                new_edge(Face::U, Color::Yellow, Face::L, Color::Green),
                new_center(Face::U, Color::Yellow),
                new_edge(Face::U, Color::Yellow, Face::R, Color::Blue),
            ],
            [
                new_corner(Face::U, Color::Yellow, Face::F, Color::Orange, Face::L, Color::Green),
                new_edge(Face::U, Color::Yellow, Face::F, Color::Orange),
                new_corner(Face::U, Color::Yellow, Face::F, Color::Orange, Face::R, Color::Blue),
            ],
        ],

        [
            [
                new_edge(Face::B, Color::Red, Face::L, Color::Green),
                new_center(Face::B, Color::Red),
                new_edge(Face::B, Color::Red, Face::R, Color::Blue),
            ],
            [
                new_center(Face::L, Color::Green),
                new_void(),
                new_center(Face::R, Color::Blue),
            ],
            [
                new_edge(Face::F, Color::Orange, Face::L, Color::Green),
                new_center(Face::F, Color::Orange),
                new_edge(Face::F, Color::Orange, Face::R, Color::Blue),
            ],
        ],

        [
            [
                new_corner(Face::D, Color::White, Face::B, Color::Red, Face::L, Color::Green),
                new_edge(Face::D, Color::White, Face::B, Color::Red),
                new_corner(Face::D, Color::White, Face::B, Color::Red, Face::R, Color::Blue),
            ],
            [
                new_edge(Face::D, Color::White, Face::L, Color::Green),
                new_center(Face::D, Color::White),
                new_edge(Face::D, Color::White, Face::R, Color::Blue),
            ],
            [
                new_corner(Face::D, Color::White, Face::F, Color::Orange, Face::L, Color::Green),
                new_edge(Face::D, Color::White, Face::F, Color::Orange),
                new_corner(Face::D, Color::White, Face::F, Color::Orange, Face::R, Color::Blue),
            ],
        ],
    ]}
}
