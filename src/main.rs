use std::{
    borrow::Borrow,
    collections::{HashMap, VecDeque},
    ops::Index,
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

    fn bfs(&self, start: &Rc<Node>) {
        if self.check_if_node_exist(start) {
            // hashmap of what nots have been visited
            let mut visited: HashMap<Rc<Node>, bool> = HashMap::new();
            for node in self.nodes.iter() {
                visited.insert(Rc::clone(&node), false);
            }

            let mut queue: VecDeque<&Rc<Node>> = VecDeque::new();

            *visited.get_mut(start).unwrap() = true;
            queue.push_back(start);

            // for (n, b) in visited.iter() {
            //     print!("{} {}, ", n.name, b);
            // }
            // println!();

            while queue.len() > 0 {
                // Dequeue the front of the queue
                let current_node = queue.pop_front().unwrap();
                print!("{} ", current_node.name);

                // get all the adj vertices of that node
                let adjs = self.edges.get(current_node).unwrap();
                for adj in adjs.iter() {
                    if !visited.get(&adj.to).unwrap() {
                        *visited.get_mut(&adj.to).unwrap() = true;
                        queue.push_back(&adj.to);
                    }
                }
            }
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
    graph.bfs(&c);
}
