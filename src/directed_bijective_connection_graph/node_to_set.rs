use crate::node_path::NodePath;
use crate::{DirectedBijectiveConnectionGraph, DirectedBijectiveConnectionGraphFunctions, Node};
use std::ops::BitXor;

impl<F> DirectedBijectiveConnectionGraph<F>
where
    F: DirectedBijectiveConnectionGraphFunctions,
{
    pub fn node_to_set(&self, src: Node, d: &[Node]) -> Vec<NodePath> {
        assert!(d.len() <= self.dimension as usize);
        assert_ne!(d.len(), 0);

        //dbg!(&d);

        if d.len() == 1 {
            if d[0] == src {
                let mut tmp = NodePath::new(self.dimension);
                tmp.push_back(src);
                return vec![tmp];
            } else {
                let mut tmp = NodePath::new(self.dimension);
                tmp.push_back(src);
                tmp.push_back(src.bitxor(1));
                return vec![tmp];
            }
        }

        let mut paths;

        let dim = d.len() as u64;
        let mask = 1 << (dim - 1);

        let all_on_same_side_as_src = d
            .iter()
            .map(|node| *node & mask)
            .all(|node_masked| node_masked == src & mask);
        if all_on_same_side_as_src {
            paths = self.node_to_set(src, &d[..dim as usize - 1]);
            paths.push(NodePath::new(self.dimension));

            debug_assert_eq!(paths.len(), dim as usize);

            let mut working_index = dim as usize - 1;

            for (index, path) in paths.iter().enumerate() {
                if let Some(pos) = path
                    .inner_path()
                    .iter()
                    .position(|&node| node == d[working_index])
                {
                    paths[index].inner_path_mut().truncate(pos);
                    paths.swap(index, dim as usize - 1);
                    working_index = index;
                    break;
                }
            }

            let phi_s = F::phi(dim, src);
            let psi_d = F::psi(dim, d[working_index]);

            let last_path = &mut paths[working_index];
            last_path.push_back(src);
            self.R_helper(dim, phi_s, psi_d, last_path);
            last_path.push_back(d[working_index]);
        } else {
            let mut same_ds = d
                .iter()
                .filter(|&node| node & mask == src & mask)
                .copied()
                .collect::<Vec<_>>();

            let mut new_d = vec![];
            let mut tmp_paths = vec![];

            for &n in d {
                if n & mask == src & mask {
                    new_d.push(n);
                    tmp_paths.push(NodePath::new(self.dimension));
                } else {
                    let dd = F::psi(dim, n);
                    if !same_ds.contains(&&dd) {
                        same_ds.push(dd);

                        new_d.push(dd);
                        tmp_paths.push({
                            let mut path = NodePath::new(self.dimension);
                            path.push_back(n);
                            path
                        });
                    } else {
                        for i in (1..dim).rev() {
                            let dd = F::psi(i, n);
                            let ddd = F::psi(dim, dd);

                            if !same_ds.contains(&&ddd) {
                                new_d.push(ddd);
                                tmp_paths.push({
                                    let mut path = NodePath::new(self.dimension);
                                    path.push_back(dd);
                                    path.push_back(n);
                                    path
                                });
                                break;
                            }
                        }
                    }
                }
            }

            debug_assert_eq!(new_d.len(), dim as usize);
            debug_assert_eq!(tmp_paths.len(), dim as usize);

            //dbg!(&new_d);
            //dbg!(&tmp_paths);

            let mut working_index = d
                .iter()
                .position(|&node| node & mask != src & mask)
                .unwrap();
            let dn = d[working_index];

            let mut path = NodePath::new(self.dimension);
            let phi_s = F::phi(dim, src);
            path.push_back(src);
            path.push_back(phi_s);
            self.R_helper(dim, phi_s, dn, &mut path);

            //dbg!(&path);

            'exit: for (index, node) in path.inner_path().iter().enumerate() {
                for (path_index, other_path) in tmp_paths.iter().enumerate() {
                    if let Some(pos) = other_path
                        .inner_path()
                        .iter()
                        .position(|other| node == other)
                    {
                        path.inner_path_mut().truncate(index);
                        path.inner_path_mut()
                            .extend(other_path.inner_path().iter().skip(pos));
                        working_index = path_index;
                        break 'exit;
                    }
                }
            }

            tmp_paths[working_index] = NodePath::new(self.dimension);

            new_d.swap(working_index, dim as usize - 1);
            let mut partial_paths = self.node_to_set(src, &new_d[..dim as usize - 1]);
            partial_paths.push(path);
            partial_paths.swap(working_index, dim as usize - 1);

            //dbg!(&partial_paths);

            partial_paths
                .iter_mut()
                .zip(tmp_paths.iter())
                .for_each(|(partial_path, path)| {
                    partial_path
                        .inner_path_mut()
                        .extend(path.inner_path().iter())
                });
            paths = partial_paths;
        }

        debug_assert_eq!(paths.len(), dim as usize);
        paths
    }
}
