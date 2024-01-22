use tobj::*;
use std::fmt::Write;

fn main() {
    let obj = load_obj("model.obj", &LoadOptions {
        single_index: true,
        triangulate: true,
        ignore_points: true,
        ignore_lines: true
    }).unwrap();

    let (models, _) = obj;

    let mut offset = 0;
    let mut vert_pos = String::new();
    let mut vert_color = String::new();
    let mut vert_norm = String::new();
    let mut tris = String::new();

    for model in models.iter() {
        println!("{}:", model.name);

        println!(" verts:");
        for (vi, vert) in model.mesh.positions.chunks(3).enumerate() {
            println!("  {vi}: {} {} {}", vert[0], vert[1], vert[2]);

            writeln!(vert_pos, "{}\n{}\n{}", vert[0], vert[1], vert[2]).unwrap();
            writeln!(vert_color, "{}", 0xE7E7E7).unwrap();
        }

        println!(" normals:");
        for (vi, norm) in model.mesh.normals.chunks(3).enumerate() {
            println!("  {vi}: {} {} {}", norm[0], norm[1], norm[2]);
            writeln!(vert_norm, "{}\n{}\n{}", norm[0], norm[1], norm[2]).unwrap();
        }

        println!(" faces:");
        for (fi, face) in model.mesh.indices.chunks(3).enumerate() {
            println!("  {fi}: {} {} {}", face[0], face[1], face[2]);

            writeln!(tris, "{}\n{}\n{}", face[0] + offset, face[1] + offset, face[2] + offset).unwrap();
        }

        offset += model.mesh.positions.len() as u32;
    }

    std::fs::write("vert_pos.txt", vert_pos).unwrap();
    std::fs::write("vert_color.txt", vert_color).unwrap();
    std::fs::write("vert_norm.txt", vert_norm).unwrap();
    std::fs::write("tris.txt", tris).unwrap();
}
