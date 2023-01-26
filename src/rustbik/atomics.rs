use strum_macros::EnumIter;


// Colors definition
pub enum Color { Yellow, White, Red, Orange, Green, Blue }

// Faces definition
#[derive(Eq, PartialEq, Hash, EnumIter)]
pub enum Face { Up, Down, Back, Front, Left, Right }
