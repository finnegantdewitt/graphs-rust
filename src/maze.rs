use std::collections::VecDeque;
use std::rc::Rc;
use std::usize;

pub type CellRef = Rc<Cell>;

// cell is a location in maze, has x, y coordinate
// 0, 0 at top left
#[derive(Eq, Debug)]
pub struct Cell {
    is_wall: bool,
    x: u32,
    y: u32,
    pub vec_coord: usize,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            is_wall: true,
            x: 0,
            y: 0,
            vec_coord: 0,
        }
    }
    fn from(is_wall: bool, x: u32, y: u32, vec_coord: usize) -> Cell {
        Cell {
            is_wall,
            x,
            y,
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
    pub width: u32,
    pub height: u32,
    cells: Vec<CellRef>,
    start: CellRef,
    end: CellRef,
}

impl Maze {
    // create a maze from image buffer
    pub fn from(image_buff: &Vec<u8>, width: u32, height: u32, is_greyscale: bool) -> Maze {
        let mut cells: Vec<CellRef> = Vec::new(); // try init with vec!
        let mut start: CellRef = Rc::new(Cell::new());
        let mut end: CellRef = Rc::new(Cell::new());

        // populate cells
        for i in 0..(width * height) {
            let mut cell = Cell::from(true, i % width, i / width, i as usize);

            let is_open_path = if is_greyscale {
                image_buff[(i) as usize] == 255
            } else {
                image_buff[(i * 3) as usize] == 255
                    && image_buff[(i * 3 + 1) as usize] == 255
                    && image_buff[(i * 3 + 2) as usize] == 255
            };

            if is_open_path {
                cell.is_wall = false;
                if cell.y == 0 {
                    let ref_cell = Rc::new(cell);
                    let clone_cell = Rc::clone(&ref_cell);
                    cells.push(ref_cell);
                    start = Rc::clone(&clone_cell);
                } else if cell.y == width - 1 {
                    let ref_cell = Rc::new(cell);
                    let clone_cell = Rc::clone(&ref_cell);
                    cells.push(ref_cell);
                    end = Rc::clone(&clone_cell);
                } else {
                    let ref_cell = Rc::new(cell);
                    cells.push(ref_cell);
                }
            } else {
                cells.push(Rc::new(cell));
            }
        }

        if start.is_wall {
            panic!("Failed to find maze start");
        }
        if end.is_wall {
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

    // gets all unvisited neighbors
    fn get_neighbors(&self, cell: &CellRef, visited_vec: &Vec<bool>) -> Vec<CellRef> {
        let mut neighbors: Vec<CellRef> = Vec::new();

        // check cell above
        if cell.y > 0 {
            let cell_above = self
                .cells
                .get(self.coords_to_vec_location(cell.x, cell.y - 1))
                .unwrap();
            if !cell_above.is_wall && !visited_vec[cell_above.vec_coord] {
                neighbors.push(Rc::clone(cell_above));
            }
        }

        // check cell below
        if cell.y < self.height - 1 {
            let cell_below = self
                .cells
                .get(self.coords_to_vec_location(cell.x, cell.y + 1))
                .unwrap();
            if !cell_below.is_wall && !visited_vec[cell_below.vec_coord] {
                neighbors.push(Rc::clone(cell_below));
            }
        }

        // check cell left
        if cell.x > 0 {
            let cell_left = self
                .cells
                .get(self.coords_to_vec_location(cell.x - 1, cell.y))
                .unwrap();
            if !cell_left.is_wall && !visited_vec[cell_left.vec_coord] {
                neighbors.push(Rc::clone(cell_left));
            }
        }

        // check cell right
        if cell.x < self.width - 1 {
            let cell_right = self
                .cells
                .get(self.coords_to_vec_location(cell.x + 1, cell.y))
                .unwrap();
            if !cell_right.is_wall && !visited_vec[cell_right.vec_coord] {
                neighbors.push(Rc::clone(cell_right));
            }
        }

        neighbors
    }

    pub fn bfs(&self) -> VecDeque<CellRef> {
        let mut path: VecDeque<CellRef> = VecDeque::new();
        let mut queue: VecDeque<CellRef> = VecDeque::new();
        // it's ok to let 0 mean "cell has no parent" because a corner cell can't be the start of the maze
        // vector matches the "cells" vector, and each value in the vector contains the index of the cells parent
        // if value is 0, cell has no parent
        let mut parent_vec: Vec<usize> = vec![0; self.cells.len()];
        let mut visited_vec: Vec<bool> = vec![false; self.cells.len()];

        visited_vec[self.start.vec_coord] = true;
        queue.push_back(Rc::clone(&self.start));

        while queue.len() > 0 {
            let current_cell = queue.pop_front().unwrap();
            for cell in self.get_neighbors(&current_cell, &visited_vec) {
                visited_vec[cell.vec_coord] = true;
                parent_vec[cell.vec_coord] = current_cell.vec_coord;
                if cell.x == self.end.x && cell.y == self.end.y {
                    break;
                }
                queue.push_back(Rc::clone(&cell));
            }
        }

        path.push_front(Rc::clone(&self.end));
        let mut cell_parent = parent_vec[self.end.vec_coord];

        while cell_parent != 0 {
            path.push_front(Rc::clone(&self.cells[cell_parent]));
            cell_parent = parent_vec[self.cells[cell_parent].vec_coord];
        }

        path
    }

    pub fn print(&self) {
        let start = &self.start;
        let end = &self.end;

        println!("width: {} height: {}", self.width, self.height);
        println!("Start {} {} end: {} {}", start.x, start.y, end.x, end.y);
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

    pub fn print_solved(&self, path: &VecDeque<CellRef>) {
        let start = &self.start;
        let end = &self.end;

        println!("width: {} height: {}", self.width, self.height);
        println!("Start {} {} end: {} {}", start.x, start.y, end.x, end.y);

        for (i, cell) in self.cells.iter().enumerate() {
            if cell.is_wall {
                print!("({}.{})", cell.x, cell.y);
            } else {
                if path.contains(cell) {
                    print!("({} {})", cell.x, cell.y);
                } else {
                    print!("({}_{})", cell.x, cell.y);
                }
            }
            if (i + 1) as u32 % self.width == 0 {
                println!();
            }
        }
    }
}
