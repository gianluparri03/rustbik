use crate::rustbik::atomics::*;

use strum::IntoEnumIterator;
use std::collections::HashMap;


pub struct Piece {
    faces: HashMap<Face, Option<Color>>,
}

// Creates a new empty piece
pub fn new_piece() -> Piece {
    let mut faces = HashMap::new();

    // Inserts all the directions in the hashmap, without a color
    for f in Face::iter() {
        faces.insert(f, None);
    }

    Piece{faces}
}
