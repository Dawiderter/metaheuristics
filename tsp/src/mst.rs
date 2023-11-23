use std::fmt::Debug;

use crate::points::Points;

pub struct Graph {
    pub n: usize,
    pub adj: Vec<Vec<(usize, u32)>>,
}

impl Graph {
    pub fn new(n: usize) -> Graph {
        Self {
            n,
            adj: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, c: u32) {
        self.adj[u].push((v,c));
        self.adj[v].push((u,c));
    }
}

pub fn mst(points: &Points) -> Graph {
    let n = points.list.len();
    let mut graph = Graph::new(n);
    let mut min_dist = vec![(usize::MAX, u32::MAX); n];

    while let Some((min_id, &(from_id, min_c))) = min_arg(&min_dist) {
        if from_id != usize::MAX {
            graph.add_edge(from_id, min_id, min_c);
        }

        min_dist[min_id] = (from_id, 0);

        let min_point = points.list[min_id];

        for (id, point) in points.list.iter().enumerate() {
            if min_point.dist(point) < min_dist[id].1 {
                min_dist[id] = (min_id, min_point.dist(point));
            }
        }
    }

    graph
}

fn min_arg(a: &[(usize, u32)]) -> Option<(usize, &(usize, u32))> {
    a.iter()
        .enumerate()
        .filter(|&(_, &(_, c))| c != 0)
        .min_by_key(|(_, &(_, c))| c)
}

pub fn dfs_cycle(graph: &Graph) -> Vec<usize> {
    let mut cycle = vec![];
    let mut stack = vec![0];
    let mut visited = vec![false; graph.n];

    visited[0] = true;
    while let Some(v) = stack.pop() {
        cycle.push(v);
        for &(u,_) in &graph.adj[v] {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            stack.push(u);
        }
    }

    cycle.push(0);
    cycle
}

pub fn measure_mst(graph: &Graph) -> u32 {
    let mut length = 0;
    let mut stack = vec![0];
    let mut visited = vec![false; graph.n];

    visited[0] = true;
    while let Some(v) = stack.pop() {
        for &(u,c) in &graph.adj[v] {
            if visited[u] {
                continue;
            }
            length += c;
            visited[u] = true;
            stack.push(u);
        }
    }

    length
}

pub fn measure_cycle(points: &Points, cycle: &[usize]) -> u32 {
    cycle
        .windows(2)
        .map(|pair| points.list[pair[0]].dist(&points.list[pair[1]]))
        .sum()
}

pub fn random_cycle(n: usize) -> Vec<usize> {
    use::rand::prelude::*;

    let mut cycle = (0..n).collect::<Vec<_>>();
    cycle.shuffle(&mut thread_rng());
    cycle.push(cycle[0]);

    cycle
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (v, list) in self.adj.iter().enumerate() {
            write!(f, "{v}: ")?;
            for (u,c) in list {
                write!(f, "(to {u}, {c}), ")?
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{points::Point, parsing::parse_problem_from_str};

    use super::*;

    fn p(x: u32, y: u32) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn mst_test() {
        let points = Points::from_points(vec![p(0, 0), p(2, 2), p(4, 4), p(2, 4)]);

        dbg!(mst(&points));
    }

    #[test]
    fn dfs_test() {
        let points = Points::from_points(vec![p(0, 0), p(2, 2), p(4, 4), p(2, 4)]);
        let mst = mst(&points);

        dbg!(dfs_cycle(&mst));
    }

    #[test]
    fn rand_test() {
        dbg!(random_cycle(10));
    }

    #[test]
    fn xqf131_test() {
        let points = parse_problem_from_str(include_str!("./../../vlsi/xqf131.tsp")).unwrap();

        let mst = mst(&points);
        let mst_length = measure_mst(&mst);
        let cycle = dfs_cycle(&mst);
        let length = measure_cycle(&points, &cycle);

        dbg!(length, mst_length);
    }

    #[test]
    fn xql662_test() {
        let points = parse_problem_from_str(include_str!("./../../vlsi/xql662.tsp")).unwrap();

        let mst = mst(&points);
        let mst_length = measure_mst(&mst);
        let cycle = dfs_cycle(&mst);
        let length = measure_cycle(&points, &cycle);

        dbg!(length, mst_length);
    }
}
