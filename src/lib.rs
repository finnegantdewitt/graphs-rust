pub mod graph;
pub mod maze;

#[cfg(test)]
mod tests {
    use super::graph::*;
    #[test]
    fn graph_test() {
        let mut graph: Graph = Graph::new();
        let a = graph.add_node(String::from("a"));
        let b = graph.add_node(String::from("b"));
        let c = graph.add_node(String::from("c"));
        let d = graph.add_node(String::from("d"));
        assert!(graph.check_if_node_exist(&a));
        // println!("does a exist {}", graph.check_if_node_exist(&a));
        graph.add_edge_by_index(0, 1, 5);
        graph.add_edge(&a, &b, 20);
        graph.add_edge(&a, &c, 2);
        graph.add_edge(&b, &c, 5);
        graph.add_edge(&c, &a, 5);
        graph.add_edge(&c, &d, 5);
        graph.add_edge(&d, &d, 5);
        let expected_bft = String::from("c a d b ");
        let mut bft_result = String::new();
        for node in graph.bft(&c).unwrap() {
            bft_result.push_str(format!("{} ", node.name).as_str());
        }
        assert!(expected_bft.eq(&bft_result));

        let expected_dft = String::from("c d a b ");
        let mut dft_result = String::new();
        for node in graph.dft(&c).unwrap() {
            dft_result.push_str(format!("{} ", node.name).as_str());
        }
        assert!(expected_dft.eq(&dft_result));
    }
}
