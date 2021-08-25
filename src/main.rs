// use lib::maze_solver::MazeSolver;
use lib::maze::Maze;
use lib::opt_maze::OptMaze;
use std::{env, process::exit};

use std::collections::VecDeque;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

//
fn test_maze_generation_speed(buf: &Vec<u8>, width: u32, height: u32, is_greyscale: bool) {
    let mut time_naive: u128 = 0;
    let runs: u128 = 1000;
    for i in 0..runs {
        let time = Instant::now();
        let mut naive_maze = Maze::from(&buf, width, height, is_greyscale);
        time_naive += time.elapsed().as_nanos();
    }
    println!(
        "average run time for naive maze for {} runs is {} nanosecs",
        runs,
        time_naive / runs
    );

    let mut time_opt: u128 = 0;
    for i in 0..runs {
        let time = Instant::now();
        let mut opt_maze = OptMaze::from(&buf, width, height, is_greyscale);
        time_opt += time.elapsed().as_nanos();
    }
    println!(
        "average run time for optim maze for {} runs is {} nanosecs",
        runs,
        time_opt / runs
    );
    println!(
        "average time difference of {}%",
        ((time_opt - time_naive) * 100) / (time_naive)
    );
}

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

    // let mut maze_solver = MazeSolver::from(img_file);
    // maze_solver.solve();
    // maze_solver.write_image(output_file);

    let decoder = png::Decoder::new(File::open(img_file).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    println!("{:?}", info);
    println!("Buffer size: {}", info.buffer_size());

    // fill the buffer
    let buff_time = Instant::now();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();
    println!(
        "Time to fill buffer:      {}",
        buff_time.elapsed().as_nanos()
    );

    // test maze load speed
    test_maze_generation_speed(
        &buf,
        info.width,
        info.height,
        info.color_type == png::ColorType::Grayscale,
    );

    // load the maze
    // let mut maze = OptMaze::from(
    //     &buf,
    //     info.width,
    //     info.height,
    //     info.color_type == png::ColorType::Grayscale,
    // );
    // maze.print();
    // maze.write_image(output_file, &mut buf, false);
}
