use lib::maze::Maze;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

fn convert_greyscale_buf_to_rgb(grey_buf: Vec<u8>) -> Vec<u8> {
    let mut color_buf: Vec<u8> = Vec::new();
    for cell in grey_buf {
        if cell == 255 {
            color_buf.push(255);
            color_buf.push(255);
            color_buf.push(255);
        } else {
            color_buf.push(0);
            color_buf.push(0);
            color_buf.push(0);
        }
    }
    color_buf
}

fn main() {
    let img_file = "perfect15k.png";
    let solved_file = "perfect15k_solved.png";

    let decoder = png::Decoder::new(File::open(img_file).unwrap());

    let (info, mut reader) = decoder.read_info().unwrap();

    println!("{:?}", info);

    let buff_time = Instant::now();
    println!("Buffer size: {}", info.buffer_size());
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();
    println!(
        "Time to fill buffer:      {}",
        buff_time.elapsed().as_nanos()
    );

    let load_time = Instant::now();

    let maze = Maze::from(
        &buf,
        info.width,
        info.height,
        info.color_type == png::ColorType::Grayscale,
    );
    // maze.print();

    println!(
        "Time to fill maze cells:  {}",
        load_time.elapsed().as_nanos()
    );
    // maze.print();
    let solve_time = Instant::now();
    let solved = maze.bfs();
    println!(
        "Time to solve maze:       {}",
        solve_time.elapsed().as_nanos()
    );

    let path = Path::new(solved_file);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let write_image_time = Instant::now();

    if info.color_type == png::ColorType::Grayscale {
        buf = convert_greyscale_buf_to_rgb(buf);
    }
    maze.apply_solved_maze_to_buf(&solved, &mut buf);

    writer.write_image_data(&buf).unwrap();
    println!(
        "Time to write image:      {}",
        write_image_time.elapsed().as_nanos()
    );
}
