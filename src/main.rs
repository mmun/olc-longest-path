extern crate rand;

use rand::prelude::*;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn dist_xy(p1: &Point, p2: &Point) -> f64 {
        f64::hypot(p2.x - p1.x, p2.y - p1.y)
    }
}

fn main() {
    const N: usize = 500; // Number of points
    const K: usize = 7; // Maximum number of nodes allowed on the path

    let points = gen_points(N);
    let distances = gen_distances(&points);

    println!("{:?}", fast_and_maybe_wrong(N, K, &points, &distances));
    // println!("{:?}", slow(N, K, &points, &distance));
}

fn fast_and_maybe_wrong(N: usize, K: usize, points: &Vec<Point>, distances: &Vec<Vec<f64>>) -> (f64, Vec<usize>) {
    // dp[k][i] is a tuple containing information about the longest path using at most `k` nodes and ending at point `i`.
    // dp[k][i].0 is the total length of the path.
    // dp[k][i].1 is the index of the previous point before point `i` (i.e. the predecessor) along the path.
    // This variable is often called "dp" for "dynamic programming", *shrug*.
    let mut dp = vec![vec![(0.0, 0); N]; K + 1];

    for k in 1..=K {
        for j in 1..N { // the point we're going to
            for i in 0..j { // the point we're coming from
                if k == K && points[i].z > points[j].z + 1000.0 {
                    continue;
                }

                let total_length = dp[k - 1][i].0 + distances[i][j];
                if dp[k][j].0 < total_length {
                    dp[k][j] = (total_length, i);
                }
            }
        }
    }

    // Find the dp data for the longest path using at most `K` segments
    let (last_point_index, _) = dp[K]
        .iter()
        .enumerate()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .expect("no solution");

    // Read the total length.
    let total_length = dp[K][last_point_index].0;

    // Read the whole path out from the rest of the dp array.
    let mut path = vec![last_point_index];

    for k in (1..K).rev() {
        let i = *path.last().unwrap();
        path.push(dp[k][i].1);

        // We've reached the beginning of the path
        if dp[k][i].0 == 0.0 {
            break;
        }
    }

    path.reverse();

    return (total_length, path);
}

fn gen_points(n: usize) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(0);

    (0..n)
        .map(|_| Point {
            x: rng.gen_range(0.0, 100000.0),
            y: rng.gen_range(0.0, 100000.0),
            z: rng.gen_range(0.0, 5000.0),
        })
        .collect::<Vec<_>>()
}

fn gen_distances(points: &Vec<Point>) -> Vec<Vec<f64>> {
    let n = points.len();
    let mut distances = vec![vec![0.0; n]; n];

    for j in 1..n {
        for i in 0..j {
            let dist = Point::dist_xy(&points[i], &points[j]);
            distances[i][j] = dist;
        }
    }

    distances
}
