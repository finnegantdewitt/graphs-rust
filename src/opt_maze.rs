use crate::graph::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::process::exit;
use std::rc::Rc;
use std::thread::current;
use std::time::Instant;
use std::vec;
type RefNode = Rc<Node>;
pub struct OptMaze {
    width: u32,
    height: u32,
    graph: Graph,
    start: RefNode,
    end: RefNode,
}

struct Neighbors {
    above: bool,
    below: bool,
    left: bool,
    right: bool,
}

fn xy_to_image_buff_location(x: u32, y: u32, height: u32) -> usize {
    ((height * y) + x) as usize
}

fn is_path(image_buff: &Vec<u8>, coord: usize, is_grey: bool) -> bool {
    if is_grey {
        image_buff[(coord) as usize] == 255
    } else {
        image_buff[(coord * 3) as usize] == 255
            && image_buff[(coord * 3 + 1) as usize] == 255
            && image_buff[(coord * 3 + 2) as usize] == 255
    }
}

fn get_neighbors(
    x: u32,
    y: u32,
    image_buff: &Vec<u8>,
    visited_vec: &Vec<bool>,
    width: u32,
    height: u32,
    is_grey: bool,
) -> Neighbors {
    let mut neighbors = Neighbors {
        above: false,
        below: false,
        left: false,
        right: false,
    };

    // check cell above
    if y > 0 {
        let cell_above_coord = xy_to_image_buff_location(x, y - 1, height);
        let cell_above = is_path(image_buff, cell_above_coord, is_grey);
        if cell_above && !visited_vec[cell_above_coord] {
            neighbors.above = true;
        }
    }

    // check cell below
    if y < height - 1 {
        let cell_below_coord = xy_to_image_buff_location(x, y + 1, height);
        let cell_below = is_path(image_buff, cell_below_coord, is_grey);
        if cell_below && !visited_vec[cell_below_coord] {
            neighbors.below = true;
        }
    }

    // check cell left
    if x > 0 {
        let cell_left_coord = xy_to_image_buff_location(x - 1, y, height);
        let cell_left = is_path(image_buff, cell_left_coord, is_grey);
        if cell_left && !visited_vec[cell_left_coord] {
            neighbors.left = true;
        }
    }

    // check cell right
    if x < width - 1 {
        let cell_right_coord = xy_to_image_buff_location(x + 1, y, height);
        let cell_right = is_path(image_buff, cell_right_coord, is_grey);
        if cell_right && !visited_vec[cell_right_coord] {
            neighbors.right = true;
        }
    }

    neighbors
}

fn convert_greyscale_buf_to_rgb(image_buffer: &mut Vec<u8>) -> Vec<u8> {
    let mut color_buf: Vec<u8> = Vec::new();
    for cell in image_buffer {
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
    color_buf
}

impl OptMaze {
    pub fn from(image_buff: &Vec<u8>, width: u32, height: u32, is_greyscale: bool) -> OptMaze {
        let mut graph = Graph::new();
        let mut start = Rc::new(Node::new());
        let mut end = Rc::new(Node::new());
        // Find Start
        for i in 0..width {
            let is_not_wall = if is_greyscale {
                image_buff[(i) as usize] == 255
            } else {
                image_buff[(i * 3) as usize] == 255
                    && image_buff[(i * 3 + 1) as usize] == 255
                    && image_buff[(i * 3 + 2) as usize] == 255
            };

            if is_not_wall {
                start = graph.add_node(i, 0, i as usize);
                break;
            }
        }
        if start.x == 0 {
            println!("Failed to find maze start");
            exit(0);
        }

        // gets all unvisited neighbors

        // breath first approach to finding nodes
        let mut node_queue = vec![Rc::clone(&start)];
        let mut visited = vec![false; (width * height) as usize];
        visited[start.vec_coord] = true;

        while !node_queue.is_empty() {
            let current_node = node_queue.pop().unwrap();
            if current_node.x == 1 && current_node.y == 3 {
                println!("hi");
            }
            let neighbors = get_neighbors(
                current_node.x,
                current_node.y,
                image_buff,
                &visited,
                width,
                height,
                is_greyscale,
            );
            println!("({} {})", current_node.x, current_node.y);

            if neighbors.left {
                let mut left_idx = 1;
                visited[xy_to_image_buff_location(current_node.x - 1, current_node.y, height)] =
                    true;
                loop {
                    println!("1");
                    let x = current_node.x - left_idx;
                    let y = current_node.y;
                    visited[xy_to_image_buff_location(x, y, height)] = true;
                    let left_neighbors =
                        get_neighbors(x, y, image_buff, &visited, width, height, is_greyscale);
                    if left_neighbors.above || left_neighbors.below || !left_neighbors.left {
                        node_queue.push(Rc::clone(&graph.add_node(
                            x,
                            y,
                            xy_to_image_buff_location(x, y, height),
                        )));
                        break;
                    } else {
                        left_idx += 1;
                    }
                }
            }
            if neighbors.right {
                let mut right_idx = 1;
                visited[xy_to_image_buff_location(current_node.x + 1, current_node.y, height)] =
                    true;
                loop {
                    println!("2");
                    let x = current_node.x + right_idx;
                    let y = current_node.y;
                    visited[xy_to_image_buff_location(x, y, height)] = true;
                    let right_neighbors =
                        get_neighbors(x, y, image_buff, &visited, width, height, is_greyscale);
                    if right_neighbors.above || right_neighbors.below || !right_neighbors.right {
                        node_queue.push(Rc::clone(&graph.add_node(
                            x,
                            y,
                            xy_to_image_buff_location(x, y, height),
                        )));
                        break;
                    } else {
                        right_idx += 1;
                    }
                }
            }
            if neighbors.above {
                let mut above_idx = 1;
                visited[xy_to_image_buff_location(current_node.x, current_node.y - 1, height)] =
                    true;
                loop {
                    println!("3");
                    let x = current_node.x;
                    let y = current_node.y - above_idx;
                    if current_node.x == 8 && current_node.y == 5 {
                        println!("hi");
                    }
                    visited[xy_to_image_buff_location(x, y, height)] = true;
                    let above_neighbors =
                        get_neighbors(x, y, image_buff, &visited, width, height, is_greyscale);
                    if above_neighbors.left || above_neighbors.right || !above_neighbors.above {
                        // check if neighbor above is visited, this should mean that that neighbor is a node
                        // in which case we should just break and not do anything
                        // this seems to only matter on the y axis, breath first would require on x axis
                        if visited[xy_to_image_buff_location(x, y - 1, height)] {
                            break;
                        }
                        node_queue.push(Rc::clone(&graph.add_node(
                            x,
                            y,
                            xy_to_image_buff_location(x, y, height),
                        )));
                        break;
                    } else {
                        above_idx += 1;
                    }
                }
            }
            if neighbors.below {
                println!("4");
                let mut below_idx = 1;
                visited[xy_to_image_buff_location(current_node.x, current_node.y + 1, height)] =
                    true;
                loop {
                    let x = current_node.x;
                    let y = current_node.y + below_idx;
                    visited[xy_to_image_buff_location(x, y, height)] = true;
                    let below_neighbors =
                        get_neighbors(x, y, image_buff, &visited, width, height, is_greyscale);
                    if below_neighbors.left || below_neighbors.right || !below_neighbors.below {
                        // check if neighbor below is visited, this should mean that that neighbor is a node
                        // in which case we should just break and not do anything
                        // this seems to only matter on the y axis, breath first would require on x axis
                        if xy_to_image_buff_location(x, y + 1, height) < (width * height) as usize
                            && visited[xy_to_image_buff_location(x, y + 1, height)]
                        {
                            break;
                        }
                        node_queue.push(Rc::clone(&graph.add_node(
                            x,
                            y,
                            xy_to_image_buff_location(x, y, height),
                        )));
                        break;
                    } else {
                        below_idx += 1;
                    }
                }
            }
        }

        OptMaze {
            width,
            height,
            graph,
            start,
            end,
        }
    }

    pub fn write_image(
        &mut self,
        filename: &String,
        image_buffer: &mut Vec<u8>,
        is_buffer_greyscale: bool,
    ) {
        let path = Path::new(filename.as_str());
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let write_image_time = Instant::now();

        // let mut img_to_write = image_buffer;

        // if is_buffer_greyscale {
        //     img_to_write = convert_greyscale_buf_to_rgb(image_buffer);
        // }

        // apply graph nodes to image
        for node in self.graph.nodes.iter() {
            image_buffer[(node.vec_coord * 3) as usize] = 0;
            image_buffer[(node.vec_coord * 3 + 1) as usize] = 0;
            image_buffer[(node.vec_coord * 3 + 2) as usize] = 255;
        }

        writer.write_image_data(image_buffer).unwrap();
        println!(
            "Time to write image:      {}",
            write_image_time.elapsed().as_nanos()
        );
    }

    pub fn print(&self) {
        self.graph.print();
    }
}
