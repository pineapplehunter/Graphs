use directed_bijectional_connection_graph::DirectedBijectiveConnectionGraph;

fn main() {
    let s = 0b000000000;
    let n = 8;

    let graph = DirectedBijectiveConnectionGraph::new_hypercube(n);

    let path = graph.lemma1(n, s);

    println!("{:#?}", path);
}