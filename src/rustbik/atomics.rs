use std::fmt;


// The colors a face can have
#[derive(Debug, Copy, Clone)]
pub enum Color { Yellow, White, Red, Orange, Green, Blue, None }

impl fmt::Display for Color {
    // Prints the letter of the color
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Color::Yellow => "Y",
            Color::White => "W",
            Color::Red => "R",
            Color::Orange => "O",
            Color::Green => "G",
            Color::Blue => "B",
            Color::None => " ",
        };

        write!(f, "{c}")
    }
}


// All the faces of a piece or a cube
#[derive(Debug, Copy, Clone)]
pub enum Face {
    U = 0,
    D = 1,
    F = 2,
    B = 3,
    L = 4,
    R = 5,
}

// The number of faces
pub const FACES_N: usize = 6;


// The axes along which a cube or a piece can be rotated
#[derive(Debug, Copy, Clone)]
pub enum Axis { X, Y, Z }
