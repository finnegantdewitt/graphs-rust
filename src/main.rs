use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
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
struct Cell {
    is_wall: bool,
    x: u32,
    y: u32,
    isVisited: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            is_wall: true,
            x: 0,
            y: 0,
            isVisited: false,
        }
    }
    fn from(is_wall: bool, x: u32, y: u32) -> Cell {
        Cell {
            is_wall,
            x,
            y,
            isVisited: false,
        }
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
    fn from(image_buff: Vec<u8>, width: u32, height: u32) -> Maze {
        let mut cells: Vec<CellRef> = Vec::new();
        let mut start: CellRef = Rc::new(RefCell::new(Cell::new()));
        let mut end: CellRef = Rc::new(RefCell::new(Cell::new()));

        // populate cells
        for i in 0..(width * height) {
            let mut cell = Cell::from(true, i % width, i / width);
            if image_buff[(i * 3) as usize] == 255
                && image_buff[(i * 3 + 1) as usize] == 255
                && image_buff[(i * 3 + 2) as usize] == 255
            {
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

    fn cell_location_in_vec(&self, cell: &CellRef) -> usize {
        ((self.height * cell.borrow().y) + cell.borrow().x) as usize
    }

    fn bfs(&self) -> Vec<CellRef> {
        let mut path: Vec<CellRef> = Vec::new();
        let mut visited = vec![false; (self.width * self.height) as usize];

        let mut queue: VecDeque<&CellRef> = VecDeque::new();

        let idx = self.cell_location_in_vec(&self.start);

        // mark start as visited
        *visited.get_mut(idx).unwrap() = true;

        queue.push_back(&self.start);

        while queue.len() > 0 {
            let current_cell = queue.pop_front().unwrap();
        }

        path
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
