pub mod graph;
pub mod maze;
pub mod opt_maze;

pub mod maze_solver {
    use super::maze::*;
    use std::collections::VecDeque;
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;
    use std::time::Instant;

    pub struct MazeSolver {
        maze: Maze,
        image_buffer: Vec<u8>,
        is_buffer_greyscale: bool,
        solved: VecDeque<CellRef>,
    }

    impl MazeSolver {
        pub fn from(filename: &String) -> MazeSolver {
            let decoder = png::Decoder::new(File::open(filename).unwrap());
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

            // load the maze
            let load_time = Instant::now();
            let maze = Maze::from(
                &buf,
                info.width,
                info.height,
                info.color_type == png::ColorType::Grayscale,
            );
            println!(
                "Time to fill maze cells:  {}",
                load_time.elapsed().as_nanos()
            );

            MazeSolver {
                maze,
                image_buffer: buf,
                is_buffer_greyscale: info.color_type == png::ColorType::Grayscale,
                solved: VecDeque::new(),
            }
        }

        pub fn solve(&mut self) {
            let solve_time = Instant::now();
            self.solved = self.maze.bfs();
            println!(
                "Time to solve maze:       {}",
                solve_time.elapsed().as_nanos()
            );
        }

        fn convert_greyscale_buf_to_rgb(&mut self) {
            let mut color_buf: Vec<u8> = Vec::new();
            for cell in &self.image_buffer {
                if *cell == 255 {
                    color_buf.push(255);
                    color_buf.push(255);
                    color_buf.push(255);
                } else {
                    color_buf.push(0);
                    color_buf.push(0);
                    color_buf.push(0);
                }
            }
            self.image_buffer = color_buf;
            self.is_buffer_greyscale = false;
        }

        fn apply_solved_maze_to_buf(&mut self) {
            for cell in self.solved.iter() {
                self.image_buffer[(cell.vec_coord * 3) as usize] = 0;
                self.image_buffer[(cell.vec_coord * 3 + 1) as usize] = 0;
                self.image_buffer[(cell.vec_coord * 3 + 2) as usize] = 255;
            }
        }

        pub fn write_image(&mut self, filename: &String) {
            let path = Path::new(filename.as_str());
            let file = File::create(path).unwrap();
            let ref mut w = BufWriter::new(file);

            let mut encoder = png::Encoder::new(w, self.maze.width, self.maze.height);
            encoder.set_color(png::ColorType::RGB);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();

            let write_image_time = Instant::now();

            if self.is_buffer_greyscale {
                self.convert_greyscale_buf_to_rgb();
            }
            self.apply_solved_maze_to_buf();

            writer.write_image_data(&self.image_buffer).unwrap();
            println!(
                "Time to write image:      {}",
                write_image_time.elapsed().as_nanos()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::graph::*;
    #[test]
    fn graph_test() {
        // let mut graph: Graph = Graph::new();
        // let a = graph.add_node(String::from("a"));
        // let b = graph.add_node(String::from("b"));
        // let c = graph.add_node(String::from("c"));
        // let d = graph.add_node(String::from("d"));
        // assert!(graph.check_if_node_exist(&a));
        // // println!("does a exist {}", graph.check_if_node_exist(&a));
        // graph.add_edge_by_index(0, 1, 5);
        // graph.add_edge(&a, &b, 20);
        // graph.add_edge(&a, &c, 2);
        // graph.add_edge(&b, &c, 5);
        // graph.add_edge(&c, &a, 5);
        // graph.add_edge(&c, &d, 5);
        // graph.add_edge(&d, &d, 5);
        // let expected_bft = String::from("c a d b ");
        // let mut bft_result = String::new();
        // for node in graph.bft(&c).unwrap() {
        //     bft_result.push_str(format!("{} ", node.name).as_str());
        // }
        // assert!(expected_bft.eq(&bft_result));

        // let expected_dft = String::from("c d a b ");
        // let mut dft_result = String::new();
        // for node in graph.dft(&c).unwrap() {
        //     dft_result.push_str(format!("{} ", node.name).as_str());
        // }
        // assert!(expected_dft.eq(&dft_result));
    }
}
