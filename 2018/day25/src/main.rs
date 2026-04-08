use anyhow::Result;
use std::io::stdin;
use text_io::try_scan;

fn main() -> Result<()> {
    let points = parse_input()?;
    let n = points.len();
    let mut uf = UnionFind::new(n);

    for i in 0..n {
        for j in (i + 1)..n {
            if manhattan(&points[i], &points[j]) <= 3 {
                uf.union(i, j);
            }
        }
    }

    let root_nodes = (0..n).filter(|&i| uf.find_tree_root(i) == i).count();
    println!("Part 1: {root_nodes}");

    Ok(())
}

fn manhattan(a: &[isize; 4], b: &[isize; 4]) -> isize {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum()
}

struct UnionFind {
    // tree[i] = parent node of i
    tree: Vec<usize>,
    // Tree depth guide to better flatten the constructed trees
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            tree: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find_tree_root(&mut self, x: usize) -> usize {
        if self.tree[x] != x {
            self.tree[x] = self.find_tree_root(self.tree[x]);
        }
        self.tree[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find_tree_root(x);
        let root_y = self.find_tree_root(y);

        if root_x == root_y {
            return;
        }

        // Extend the shortest branch
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => self.tree[root_x] = root_y,
            std::cmp::Ordering::Greater => self.tree[root_y] = root_x,
            std::cmp::Ordering::Equal => {
                self.tree[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

fn parse_input() -> Result<Vec<[isize; 4]>> {
    stdin()
        .lines()
        .map(|l| {
            let l = l?;
            let (a, b, c, d): (isize, isize, isize, isize);
            try_scan!(l.trim().bytes() => "{},{},{},{}", a, b, c, d);
            Ok([a, b, c, d])
        })
        .collect()
}
