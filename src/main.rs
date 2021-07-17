use std::cell::RefCell;
use std::collections::VecDeque;
use std::hash::Hash;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use std::{fs::File, usize};

mod graph;
use crate::graph::{Graph, Node};

type CellRef = Rc<RefCell<Cell>>;

fn print_node_vec(vec: Vec<Rc<Node>>) {
    for node in vec {
        print!("{} ", node.name);
    }
    println!();
}

// cell is a location in maze, has x, y coordinate
// 0, 0 at top left
#[derive(Eq, Hash, Debug)]
struct Cell {
    is_wall: bool,
    is_visited: bool,
    x: u32,
    y: u32,
    vec_coord: usize,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            is_wall: true,
            x: 0,
            y: 0,
            is_visited: false,
            vec_coord: 0,
        }
    }
    fn from(is_wall: bool, x: u32, y: u32, vec_coord: usize) -> Cell {
        Cell {
            is_wall,
            x,
            y,
            is_visited: false,
            vec_coord,
        }
    }
}
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Maze {
    width: u32,
    height: u32,
    cells: Vec<CellRef>,
    start: CellRef,
    end: CellRef,
}

impl Maze {
    // create a maze from image buffer
    fn from(image_buff: &Vec<u8>, width: u32, height: u32, is_greyscale: bool) -> Maze {
        let mut cells: Vec<CellRef> = Vec::new(); // try init with vec!
        let mut start: CellRef = Rc::new(RefCell::new(Cell::new()));
        let mut end: CellRef = Rc::new(RefCell::new(Cell::new()));

        // populate cells
        for i in 0..(width * height) {
            //println!("{}", i);
            let mut cell = Cell::from(true, i % width, i / width, i as usize);
            let mut is_open_path = false;

            if is_greyscale {
                is_open_path = image_buff[(i) as usize] == 255;
            } else {
                is_open_path = image_buff[(i * 3) as usize] == 255
                    && image_buff[(i * 3 + 1) as usize] == 255
                    && image_buff[(i * 3 + 2) as usize] == 255;
            }

            if is_open_path {
                cell.is_wall = false;
                if cell.y == 0 {
                    let ref_cell = Rc::new(RefCell::new(cell));
                    let clone_cell = Rc::clone(&ref_cell);
                    cells.push(ref_cell);
                    start = Rc::clone(&clone_cell);
                } else if cell.y == width - 1 {
                    let ref_cell = Rc::new(RefCell::new(cell));
                    let clone_cell = Rc::clone(&ref_cell);
                    cells.push(ref_cell);
                    end = Rc::clone(&clone_cell);
                } else {
                    let ref_cell = Rc::new(RefCell::new(cell));
                    cells.push(ref_cell);
                }
            } else {
                cells.push(Rc::new(RefCell::new(cell)));
            }
        }

        if start.borrow().is_wall {
            panic!("Failed to find maze start");
        }
        if end.borrow().is_wall {
            panic!("Failed to find the exit of the maze");
        }

        Maze {
            width,
            height,
            cells,
            start,
            end,
        }
    }

    fn coords_to_vec_location(&self, x: u32, y: u32) -> usize {
        ((self.height * y) + x) as usize
    }

    fn cell_location_in_vec(&self, cell: &CellRef) -> usize {
        self.coords_to_vec_location(cell.borrow().x, cell.borrow().y)
    }

    // gets all unvisited neighbors
    fn get_neighbors(&self, cell: &CellRef) -> Vec<CellRef> {
        let mut neighbors: Vec<CellRef> = Vec::new();

        let cell = cell.borrow();

        // check cell above
        if cell.y > 0 {
            let cell_above = self
                .cells
                .get(self.coords_to_vec_location(cell.x, cell.y - 1))
                .unwrap();
            if !cell_above.borrow().is_wall && !cell_above.borrow().is_visited {
                neighbors.push(Rc::clone(cell_above));
            }
        }

        // check cell below
        if cell.y < self.height - 1 {
            let cell_below = self
                .cells
                .get(self.coords_to_vec_location(cell.x, cell.y + 1))
                .unwrap();
            if !cell_below.borrow().is_wall && !cell_below.borrow().is_visited {
                neighbors.push(Rc::clone(cell_below));
            }
        }

        // check cell left
        if cell.x > 0 {
            let cell_left = self
                .cells
                .get(self.coords_to_vec_location(cell.x - 1, cell.y))
                .unwrap();
            if !cell_left.borrow().is_wall && !cell_left.borrow().is_visited {
                neighbors.push(Rc::clone(cell_left));
            }
        }

        // check cell right
        if cell.x < self.width - 1 {
            let cell_right = self
                .cells
                .get(self.coords_to_vec_location(cell.x + 1, cell.y))
                .unwrap();
            if !cell_right.borrow().is_wall && !cell_right.borrow().is_visited {
                neighbors.push(Rc::clone(cell_right));
            }
        }

        neighbors
    }

    fn bfs(&self) -> VecDeque<CellRef> {
        let mut path: VecDeque<CellRef> = VecDeque::new();
        let mut queue: VecDeque<CellRef> = VecDeque::new();
        // it's ok to let 0 mean "cell has no parent" because a corner cell can't be the start of the maze
        // vector matches the "cells" vector, and each value in the vector contains the index of the cells parent
        // if value is 0, cell has no parent
        let mut parent_vec: Vec<usize> = vec![0; self.cells.len()];

        self.start.borrow_mut().is_visited = true;
        queue.push_back(Rc::clone(&self.start));

        while queue.len() > 0 {
            let current_cell = queue.pop_front().unwrap();
            current_cell.borrow_mut().is_visited = true;
            for cell in self.get_neighbors(&current_cell) {
                parent_vec[cell.borrow().vec_coord] = current_cell.borrow().vec_coord;
                if cell.borrow().x == self.end.borrow().x && cell.borrow().y == self.end.borrow().y
                {
                    break;
                }
                queue.push_back(Rc::clone(&cell));
            }
        }

        path.push_front(Rc::clone(&self.end));
        let mut cell_parent = parent_vec[self.end.borrow().vec_coord];

        while cell_parent != 0 {
            path.push_front(Rc::clone(&self.cells[cell_parent]));
            cell_parent = parent_vec[self.cells[cell_parent].borrow().vec_coord];
        }
        // self.print_solved(&path);
        path
    }

    fn apply_solved_maze_to_buf(&self, solved_path: &VecDeque<CellRef>, buf: &mut Vec<u8>) {
        for solved_cell in solved_path.iter() {
            let cell = solved_cell.borrow();
            buf[(cell.vec_coord * 3) as usize] = 0;
            buf[(cell.vec_coord * 3 + 1) as usize] = 255;
            buf[(cell.vec_coord * 3 + 2) as usize] = 255;
        }
    }

    fn print(&self) {
        let start = self.start.borrow();
        let end = self.end.borrow();

        println!("width: {} height: {}", self.width, self.height);
        println!("Start {} {} end: {} {}", start.x, start.y, end.x, end.y);
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.borrow().is_wall {
                //print!("0")
                print!("({}.{})", cell.borrow().x, cell.borrow().y);
            } else {
                //print!("1");
                print!("({}_{})", cell.borrow().x, cell.borrow().y);
            }
            if (i + 1) as u32 % self.width == 0 {
                println!();
            }
        }
    }

    fn print_solved(&self, path: &VecDeque<CellRef>) {
        let start = self.start.borrow();
        let end = self.end.borrow();

        println!("width: {} height: {}", self.width, self.height);
        println!("Start {} {} end: {} {}", start.x, start.y, end.x, end.y);

        for (i, cell) in self.cells.iter().enumerate() {
            if cell.borrow().is_wall {
                print!("({}.{})", cell.borrow().x, cell.borrow().y);
            } else {
                if path.contains(cell) {
                    print!("({} {})", cell.borrow().x, cell.borrow().y);
                } else {
                    print!("({}_{})", cell.borrow().x, cell.borrow().y);
                }
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

    let img_file = "5k.png";
    let solved_file = "5k_solved.png";

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
    maze.apply_solved_maze_to_buf(&solved, &mut buf);

    writer.write_image_data(&buf).unwrap();
    println!(
        "Time to write image:      {}",
        write_image_time.elapsed().as_nanos()
    );

    // for cell in solved.iter() {
    //     println!("{} {}", cell.borrow().x, cell.borrow().y);
    // }
}
