use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

pub struct Edge {
    weight: u32,
    from: Rc<Node>,
    to: Rc<Node>,
}

impl Edge {
    fn from(from: Rc<Node>, to: Rc<Node>, weight: u32) -> Edge {
        Edge { from, to, weight }
    }
}

#[derive(Eq, Hash)]
pub struct Node {
    name: String,
}
impl Node {
    fn from(name: String) -> Node {
        Node { name }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub struct Graph {
    nodes: Vec<Rc<Node>>,
    edges: HashMap<Rc<Node>, Vec<Edge>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, s: String) -> Rc<Node> {
        let new_node = Rc::new(Node::from(s));
        let return_node = Rc::clone(&new_node);
        self.nodes.push(new_node);
        return_node
    }

    fn check_if_node_exist(&self, node: &Rc<Node>) -> bool {
        self.nodes.contains(node)
    }

    // if you add an edge with the same from and to node, it overwrites the weight
    fn add_edge(&mut self, from: &Rc<Node>, to: &Rc<Node>, weight: u32) {
        if self.check_if_node_exist(from) && self.check_if_node_exist(to) {
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
    }

    // if you add an edge with the same from and to node, it overwrites the weight
    fn add_edge_by_index(&mut self, from: usize, to: usize, weight: u32) {
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
    fn bft(&self, start: &Rc<Node>) -> Option<Vec<Rc<Node>>> {
        if self.check_if_node_exist(start) {
            let mut vec_to_return: Vec<Rc<Node>> = Vec::new();

            // hashmap of what nots have been visited
            let mut visited: HashMap<&Rc<Node>, bool> = HashMap::new();
            for node in self.nodes.iter() {
                visited.insert(node, false);
            }

            let mut queue: VecDeque<&Rc<Node>> = VecDeque::new();

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

    fn dft(&self, start: &Rc<Node>) -> Option<Vec<Rc<Node>>> {
        if self.check_if_node_exist(start) {
            let mut vec_to_return: Vec<Rc<Node>> = Vec::new();

            let mut visited: HashMap<&Rc<Node>, bool> = HashMap::new();
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

    fn print(&self) {
        for (index, node) in self.nodes.iter().enumerate() {
            print!("{}: {}: ", index, node.name);
            if self.edges.contains_key(node) {
                for edge in self.edges.get(node).expect("Failed to unwrap edges") {
                    print!("{}->({}){} ", edge.from.name, edge.weight, edge.to.name);
                }
            }
            println!();
        }
    }
}

fn print_node_vec(vec: Vec<Rc<Node>>) {
    for node in vec {
        print!("{} ", node.name);
    }
    println!();
}

fn factorial(n: i64) -> i64 {
    if n == 1 {
        n
    } else {
        n * factorial(n - 1)
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

    println!("Recursion test 3!={} 6!={}", factorial(3), factorial(6));

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
}
