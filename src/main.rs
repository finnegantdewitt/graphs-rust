use lib::maze_solver::MazeSolver;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Need to enter an input filename");
        exit(1);
    }

    let img_file = &args[1];
    let mut out = String::clone(&img_file);

    if !img_file.contains(".png") {
        println!("input file must be a png");
        exit(1);
    }
    let output_file = if args.len() == 2 {
        let png_location = out
            .find(".png")
            .expect("Couldn't find .png?? how did i get here");
        out.insert_str(png_location, "_solved");
        &out
    } else {
        &args[2]
    };

    let mut maze_solver = MazeSolver::from(img_file);
    maze_solver.solve();
    maze_solver.write_image(output_file);
}
