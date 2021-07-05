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

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Node {
    fn from(name: String) -> Node {
        Node { name }
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

    fn add_edge(&mut self, from: &Rc<Node>, to: &Rc<Node>, weight: u32) {
        if self.check_if_node_exist(from) && self.check_if_node_exist(to) {
            if !self.edges.contains_key(from) {
                self.edges.insert(
                    Rc::clone(from),
                    vec![Edge::from(Rc::clone(from), Rc::clone(to), weight)],
                );
            } else {
                self.edges
                    .get_mut(from)
                    .expect("failed to unwrap vec in add_edge")
                    .push(Edge::from(Rc::clone(from), Rc::clone(to), weight))
            }
        }
    }

    fn add_edge_by_index(&mut self, from: usize, to: usize, weight: u32) {
        let from_node = self.nodes.get(from).expect("Failed to get from node");
        let to_node = self.nodes.get(to).expect("Failed to get to node");
        if !self.edges.contains_key(from_node) {
            self.edges.insert(
                Rc::clone(from_node),
                vec![Edge::from(Rc::clone(from_node), Rc::clone(to_node), weight)],
            );
        } else {
            self.edges
                .get_mut(from_node)
                .expect("failed to unwrap vec in add_edge_by_index")
                .push(Edge::from(Rc::clone(from_node), Rc::clone(to_node), weight))
        }
    }

    fn BFS(&self, start: &Rc<Node>) {
        if self.check_if_node_exist(start) {
            let mut visited = vec![false; self.nodes.len()];
            let mut queue: VecDeque<&Rc<Node>> = VecDeque::new();
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

fn main() {
    let mut graph: Graph = Graph::new();
    let a = graph.add_node(String::from("a"));
    let b = graph.add_node(String::from("b"));
    let c = graph.add_node(String::from("c"));
    println!("does a exist {}", graph.check_if_node_exist(&a));
    graph.add_edge_by_index(0, 1, 32);
    graph.add_edge(&a, &b, 20);
    graph.add_edge(&b, &c, 2);
    graph.print();
}
