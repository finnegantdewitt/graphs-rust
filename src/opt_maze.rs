use crate::graph::*;
use std::process::exit;
use std::rc::Rc;
use std::vec;
type RefNode = Rc<Node>;
struct OptMaze {
    width: u32,
    height: u32,
    graph: Graph,
    start: RefNode,
    end: RefNode,
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

        fn check_buff(vec_coord: usize, image_buff: &Vec<u8>) {
            
        }

        // breath first approach to finding nodes
        let mut node_queue = vec![&start];
        while !node_queue.is_empty() {
            let current_node = node_queue.pop().unwrap();
            if current_node.x
        }

        OptMaze {
            width,
            height,
            graph,
            start,
            end,
        }
    }
}
