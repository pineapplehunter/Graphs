#![feature(test)]

extern crate test;
use gt_directed_bijective_connection_graph::{
    DirectedBijectiveConnectionGraph, NPathsToNode, NodeToNodeDisjointPaths,
    NodeToSetDisjointPaths, SinglePath,
};
use gt_graph::{Dims, Graph, Node};
use test::Bencher;

#[bench]
fn custom_function_lemma1_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let d = 0xffff_ffff_0000_0000;

    b.iter(|| graph.n_paths_to_node(dim, d));
}

#[bench]
fn custom_function_lemma2_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let s = 0x0000_0000_ffff_ffff;
    let d = 0xffff_ffff_0000_0000;

    b.iter(|| graph.single_path(s, d));
}

#[bench]
fn custom_function_node_to_set_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let s = 0x0000_0000_ffff_ffff;
    let mut d = vec![];

    for i in 0..64 {
        d.push(1 << i);
    }

    b.iter(|| graph.node_to_set_disjoint_paths(s, &d));
}

#[bench]
fn custom_function_node_to_node_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let s = 0x0000_0000_ffff_ffff;
    let d = 0xffff_ffff_0000_0000;
    b.iter(|| graph.node_to_node_disjoint_paths(s, d));
}

struct CustomFunctionGraph(Dims);

impl CustomFunctionGraph {
    pub fn new(n: Dims) -> Self {
        Self(n)
    }
}

impl Graph for CustomFunctionGraph {
    #[inline(always)]
    fn dimension(&self) -> u64 {
        self.0
    }

    fn phi(&self, n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask == 0 {
            (1 << (n - 1)) ^ (node)
        } else {
            (u64::max_value() << (n)) ^ (u64::max_value()) ^ (node)
        }
    }
}

impl DirectedBijectiveConnectionGraph for CustomFunctionGraph {
    fn psi(&self, n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask != 0 {
            (1 << (n - 1)) ^ (node)
        } else {
            (u64::max_value() << (n)) ^ (u64::max_value()) ^ (node)
        }
    }
}
