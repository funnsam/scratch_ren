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

    for (mi, model) in models.iter().enumerate() {
        let mut vert_pos = String::new();
        let mut vert_color = String::new();
        let mut tris = String::new();

        println!("{mi}:");
        println!(" name: {}", model.name);

        println!(" verts:");
        for (vi, vert) in model.mesh.positions.chunks(3).enumerate() {
            println!("  {vi}: {} {} {}", vert[0], vert[1], vert[2]);

            writeln!(vert_pos, "{}\n{}\n{}", vert[0], vert[1], vert[2]).unwrap();
            writeln!(vert_color, "{}", 0xEEEEEE).unwrap();
        }

        println!(" normals:");
        for (vi, vert) in model.mesh.normals.chunks(3).enumerate() {
            println!("  {vi}: {} {} {}", vert[0], vert[1], vert[2]);
        }

        println!(" faces:");
        for (fi, face) in model.mesh.indices.chunks(3).enumerate() {
            println!("  {fi}: {} {} {}", face[0], face[1], face[2]);

            writeln!(tris, "{}\n{}\n{}", face[0], face[1], face[2]).unwrap();
        }

        std::fs::write(format!("m_{}_vert_pos.txt", model.name), vert_pos).unwrap();
        std::fs::write(format!("m_{}_vert_color.txt", model.name), vert_color).unwrap();
        std::fs::write(format!("m_{}_tris.txt", model.name), tris).unwrap();
    }
}
