use crate::rustbik::atomics::*;
use crate::rustbik::pieces::*;


pub struct Cube {
    corners: [Piece; 8],
    edges: [Piece; 12],
    centers: [Piece; 6],
}

// Creates a new, solved cube
pub fn new_cube() -> Cube {
    // Builds the centers
    let centers: [Piece; 6] = [
        new_center(Face::Up, Color::Yellow),
        new_center(Face::Down, Color::White),
        new_center(Face::Back, Color::Red),
        new_center(Face::Front, Color::Orange),
        new_center(Face::Left, Color::Green),
        new_center(Face::Right, Color::Blue)];

    // Builds the edges
    let edges: [Piece; 12] = [
        new_edge(Face::Up, Color::Yellow, Face::Back, Color::Red),
        new_edge(Face::Up, Color::Yellow, Face::Left, Color::Green),
        new_edge(Face::Up, Color::Yellow, Face::Right, Color::Blue),
        new_edge(Face::Up, Color::Yellow, Face::Front, Color::Orange),
        new_edge(Face::Back, Color::Red, Face::Left, Color::Green),
        new_edge(Face::Back, Color::Red, Face::Right, Color::Blue),
        new_edge(Face::Front, Color::Orange, Face::Left, Color::Green),
        new_edge(Face::Front, Color::Orange, Face::Right, Color::Blue),
        new_edge(Face::Down, Color::White, Face::Back, Color::Red),
        new_edge(Face::Down, Color::White, Face::Left, Color::Green),
        new_edge(Face::Down, Color::White, Face::Right, Color::Blue),
        new_edge(Face::Down, Color::White, Face::Front, Color::Orange)];

    // Builds the corners
    let corners: [Piece; 8] = [
        new_corner(Face::Up, Color::Yellow, Face::Back, Color::Red, Face::Left, Color::Green),
        new_corner(Face::Up, Color::Yellow, Face::Back, Color::Red, Face::Right, Color::Blue),
        new_corner(Face::Up, Color::Yellow, Face::Front, Color::Orange, Face::Left, Color::Green),
        new_corner(Face::Up, Color::Yellow, Face::Front, Color::Orange, Face::Right, Color::Blue),
        new_corner(Face::Down, Color::White, Face::Back, Color::Red, Face::Left, Color::Green),
        new_corner(Face::Down, Color::White, Face::Back, Color::Red, Face::Right, Color::Blue),
        new_corner(Face::Down, Color::White, Face::Front, Color::Orange, Face::Left, Color::Green),
        new_corner(Face::Down, Color::White, Face::Front, Color::Orange, Face::Right, Color::Blue)];

    // Creates the cube
    Cube{corners, edges, centers}
}
