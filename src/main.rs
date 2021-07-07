use std::rc::Rc;

mod graph;
use crate::graph::{Graph, Node};

fn print_node_vec(vec: Vec<Rc<Node>>) {
    for node in vec {
        print!("{} ", node.name);
    }
    println!();
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
