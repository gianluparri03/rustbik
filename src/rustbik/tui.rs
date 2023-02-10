use crate::rustbik::atomics::*;
use crate::rustbik::cube::Cube;


fn print_faces_row(c: &Cube, faces: Vec<Face>) {
    // Calculates the border and the padding
    let border = "+---+---+---+ ".repeat(faces.len());
    let padding = if faces.len() < 2 { " ".repeat(14) } else { "".to_string() };

    // Gets the cube's faces
    let faces = faces.iter().map(|f| c.get_face(*f)).collect::<Vec<_>>();

    // Prints all the faces in the row
    for i in 0..3 {
        println!("{}{}", padding, border);

        if faces.len() < 2 {
            print!("{}", padding);
        }
        for j in 0..faces.len() {
            print!("{}", format!("| {} | {} | {} | ", faces[j][i*3], faces[j][i*3 + 1], faces[j][i*3 + 2]));
        }

        println!("");
    }

    println!("{}{}", padding, border);
}

pub fn print_cube(c: &Cube) {
    // Prints the cube
    print_faces_row(c, vec![Face::U]);
    print_faces_row(c, vec![Face::L, Face::F, Face::R, Face::B]);
    print_faces_row(c, vec![Face::D]);
}
