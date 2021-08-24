use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

type RefNode = Rc<Node>;

pub struct Edge {
    pub weight: u32,
    pub from: RefNode,
    pub to: RefNode,
}

impl Edge {
    pub fn from(from: RefNode, to: RefNode, weight: u32) -> Edge {
        Edge { from, to, weight }
    }
}

#[derive(Eq, Hash)]
pub struct Node {
    pub x: u32,
    pub y: u32,
    pub vec_coord: usize,
}
impl Node {
    pub fn new() -> Node {
        Node {
            x: 0,
            y: 0,
            vec_coord: 0,
        }
    }
    pub fn from(x: u32, y: u32, vec_coord: usize) -> Node {
        Node { x, y, vec_coord }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.vec_coord == other.vec_coord
    }
}

pub struct Graph {
    pub nodes: Vec<RefNode>,
    pub edges: HashMap<RefNode, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, x: u32, y: u32, vec_coord: usize) -> RefNode {
        let new_node = Rc::new(Node::from(x, y, vec_coord));
        let return_node = Rc::clone(&new_node);
        self.nodes.push(new_node);
        return_node
    }

    pub fn find_node_xy(&self, x: u32, y: u32) -> Option<RefNode> {
        for node in &self.nodes {
            if node.x == x && node.y == y {
                return Some(Rc::clone(node));
            }
        }
        return None;
    }

    pub fn check_if_node_exist(&self, node: &RefNode) -> bool {
        self.nodes.contains(node)
    }

    // if you add an edge with the same from and to node, it overwrites the weight
    pub fn add_edge(&mut self, from: &RefNode, to: &RefNode, weight: u32) {
        if !self.edges.contains_key(from) {
            self.edges.insert(
                Rc::clone(from),
                vec![Edge::from(Rc::clone(from), Rc::clone(to), weight)],
            );
        } else {
            let edge_vec = self
                .edges
                .get_mut(from)
                .expect("failed to unwrap vec in add_edge");
            // if it finds this edge already exist, it overwrites it's weight and returns
            for edge in edge_vec.iter_mut() {
                if &edge.to == to {
                    edge.weight = weight;
                    return;
                }
            }
            edge_vec.push(Edge::from(Rc::clone(from), Rc::clone(to), weight))
        }
    }

    // if you add an edge with the same from and to node, it overwrites the weight
    pub fn add_edge_by_index(&mut self, from: usize, to: usize, weight: u32) {
        let from_node = self.nodes.get(from).expect("Failed to get from node");
        let to_node = self.nodes.get(to).expect("Failed to get to node");
        if !self.edges.contains_key(from_node) {
            self.edges.insert(
                Rc::clone(from_node),
                vec![Edge::from(Rc::clone(from_node), Rc::clone(to_node), weight)],
            );
        } else {
            let edge_vec = self
                .edges
                .get_mut(from_node)
                .expect("failed to unwrap vec in add_edge");
            // if it finds this edge already exist, it overwrites it's weight and returns
            for edge in edge_vec.iter_mut() {
                if &edge.to == to_node {
                    edge.weight = weight;
                    return;
                }
            }
            edge_vec.push(Edge::from(Rc::clone(from_node), Rc::clone(to_node), weight))
        }
    }

    // Breath first tree traversal
    pub fn bft(&self, start: &RefNode) -> Option<Vec<RefNode>> {
        if self.check_if_node_exist(start) {
            let mut vec_to_return: Vec<RefNode> = Vec::new();

            // hashmap of what nots have been visited
            let mut visited: HashMap<&RefNode, bool> = HashMap::new();
            for node in self.nodes.iter() {
                visited.insert(node, false);
            }

            let mut queue: VecDeque<&RefNode> = VecDeque::new();

            *visited.get_mut(start).unwrap() = true;
            queue.push_back(start);

            while queue.len() > 0 {
                // Dequeue the front of the queue
                let current_node = queue.pop_front().unwrap();
                vec_to_return.push(Rc::clone(current_node));

                // get all the adj vertices of that node
                let adjs = self.edges.get(current_node);
                match adjs {
                    None => {}
                    Some(adjs) => {
                        for adj in adjs.iter() {
                            if !visited.get(&adj.to).unwrap() {
                                *visited.get_mut(&adj.to).unwrap() = true;
                                queue.push_back(&adj.to);
                            }
                        }
                    }
                }
            }
            Some(vec_to_return)
        } else {
            None
        }
    }

    pub fn dft(&self, start: &RefNode) -> Option<Vec<RefNode>> {
        if self.check_if_node_exist(start) {
            let mut vec_to_return: Vec<RefNode> = Vec::new();

            let mut visited: HashMap<&RefNode, bool> = HashMap::new();
            for node in self.nodes.iter() {
                visited.insert(node, false);
            }

            let mut stack = Vec::new();
            stack.push(start);

            while stack.len() > 0 {
                let current_node = stack.pop().unwrap();
                if !visited.get(&current_node).unwrap() {
                    vec_to_return.push(Rc::clone(current_node));
                    *visited.get_mut(&current_node).unwrap() = true;
                    match self.edges.get(current_node) {
                        None => {}
                        Some(edges) => {
                            for edge in edges {
                                stack.push(&edge.to);
                            }
                        }
                    }
                }
            }

            Some(vec_to_return)
        } else {
            None
        }
    }

    pub fn print(&self) {
        for (index, node) in self.nodes.iter().enumerate() {
            print!("{}: {},{}: ", index, node.x, node.y);
            if self.edges.contains_key(node) {
                for edge in self.edges.get(node).expect("Failed to unwrap edges") {
                    print!(
                        "{},{}->({}){},{} ",
                        edge.from.x, edge.from.y, edge.weight, edge.to.x, edge.to.y
                    );
                }
            }
            println!();
        }
    }
}
