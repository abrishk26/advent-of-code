use std::fs;

type Pos = (i64, i64, i64);

struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
    circuits: usize
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            size: vec![1; n],
            circuits: n
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.par[x] != x {
            self.par[x] = self.par[self.par[x]];
            x = self.par[x];
        }
        return x;
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let par_x = self.find(x);
        let par_y = self.find(y);

        if par_x == par_y {
            return false;
        };

        if self.size[par_x] >= self.size[par_y] {
            self.par[par_y] = par_x;
            self.size[par_x] += self.size[par_y];
        } else {
            self.par[par_x] = par_y;
            self.size[par_y] += self.size[par_x];
        }
        self.circuits -= 1; 
        return true;
    }
}

fn get_dist(first: Pos, second: Pos) -> f64 {
    let (x1, y1, z1) = first;
    let (x2, y2, z2) = second;

    let dx = x1 - x2;
    let dy = y1 - y2;
    let dz = z1 - z2;

    return (dx * dx + dy * dy + dz * dz) as f64;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let coordinates: Vec<Pos> = lines
        .iter()
        .map(|line| {
            let c: Vec<i64> = line.split(',').map(|v| v.parse().unwrap()).collect();
            (c[0], c[1], c[2])
        })
        .collect();

    let n = coordinates.len();
    let mut distances: Vec<(f64, usize, usize)> = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            let curr_dist = get_dist(coordinates[i], coordinates[j]);
            distances.push((curr_dist, i, j));
        }
    }

    let mut uf = UnionFind::new(n);
    distances.sort_by(|a, b| (a.0).total_cmp(&b.0));

    // let limit = 1000;
    let len = distances.len();

    for i in 0..len {
        let (_, first_ind, second_ind) = distances[i];
        if uf.union(first_ind, second_ind) {
            if uf.circuits == 1 { 
                println!("{}", coordinates[first_ind].0 * coordinates[second_ind].0); 
            } 
        };
    }
}
