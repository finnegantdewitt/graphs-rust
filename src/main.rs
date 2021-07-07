use std::rc::Rc;
use std::{fs::File, usize};

mod graph;
use crate::graph::{Graph, Node};

fn print_node_vec(vec: Vec<Rc<Node>>) {
    for node in vec {
        print!("{} ", node.name);
    }
    println!();
}

// cell is a location in maze, has x, y coordinate
// 0, 0 at top left
struct Cell {
    is_wall: bool,
    x: u32,
    y: u32,
}

pub struct Maze {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Maze {
    // create a maze from image buffer
    fn from(image_buff: Vec<u8>, width: u32, height: u32) -> Maze {
        let mut cells: Vec<Cell> = Vec::new();

        // populate cells
        for i in 0..(width * height) {
            let mut cell = Cell {
                is_wall: true,
                x: i % width,
                y: i / width,
            };
            if image_buff[(i * 3) as usize] == 255
                && image_buff[(i * 3 + 1) as usize] == 255
                && image_buff[(i * 3 + 2) as usize] == 255
            {
                cell.is_wall = false;
                cells.push(cell);
            } else {
                cells.push(cell);
            }
        }

        Maze {
            width,
            height,
            cells,
        }
    }

    fn print(&self) {
        println!("width: {} height: {}", self.width, self.height);
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.is_wall {
                //print!("0")
                print!("({}.{})", cell.x, cell.y);
            } else {
                //print!("1");
                print!("({}_{})", cell.x, cell.y);
            }
            if (i + 1) as u32 % self.width == 0 {
                println!();
            }
        }
    }
}

fn main() {
    let mut graph: Graph = Graph::new();
    let a = graph.add_node(String::from("a"));
    let b = graph.add_node(String::from("b"));
    let c = graph.add_node(String::from("c"));
    let d = graph.add_node(String::from("d"));
    println!("does a exist {}", graph.check_if_node_exist(&a));
    graph.add_edge_by_index(0, 1, 5);
    graph.add_edge(&a, &b, 20);
    graph.add_edge(&a, &c, 2);
    graph.add_edge(&b, &c, 5);
    graph.add_edge(&c, &a, 5);
    graph.add_edge(&c, &d, 5);
    graph.add_edge(&d, &d, 5);
    graph.print();

    let bft_result = graph.bft(&c);
    match bft_result {
        None => {
            println!("Failed to create bft graph");
        }
        Some(bft) => {
            println!("BFT: ");
            print_node_vec(bft);
        }
    }

    let dft_result = graph.dft(&c);
    match dft_result {
        None => {
            println!("Failed to create dft graph");
        }
        Some(dft) => {
            println!("DFT: ");
            print_node_vec(dft);
        }
    }

    let decoder = png::Decoder::new(File::open("tiny.png").unwrap());

    let (info, mut reader) = decoder.read_info().unwrap();

    println!("{:?}", info);

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();
    println!("{:?}", buf);

    let maze = Maze::from(buf, info.width, info.height);
    maze.print();
}
