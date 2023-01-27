#[derive(Copy, Clone)]
pub enum Color { Yellow, White, Red, Orange, Green, Blue }

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Face { Up, Down, Front, Back, Left, Right }
