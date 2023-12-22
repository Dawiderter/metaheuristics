use crate::{mst::measure_perm, points::Points};
use rand::{distributions::Uniform, prelude::*};

pub fn wrapped_dist(points: &Points, perm: &[usize], a: isize, b: isize) -> i32 {
    let n = perm.len() as isize;

    let a_wrapped = ((a % n + n) % n) as usize;
    let b_wrapped = ((b % n + n) % n) as usize;

    points.list[perm[a_wrapped]].dist(&points.list[perm[b_wrapped]]) as i32
}

pub fn inv_diff(points: &Points, perm: &[usize], inv: (usize, usize)) -> i32 {
    if (inv.0, inv.1) == (0, perm.len() - 1) {
        return 0;
    }

    let inv_left = inv.0 as isize;
    let inv_right = inv.1 as isize;

    -wrapped_dist(points, perm, inv_left - 1, inv_left)
        + wrapped_dist(points, perm, inv_left - 1, inv_right)
        - wrapped_dist(points, perm, inv_right, inv_right + 1)
        + wrapped_dist(points, perm, inv_left, inv_right + 1)
}

pub fn invert(perm: &mut [usize], inv: (usize, usize)) {
    perm[inv.0..=inv.1].reverse();
}

pub fn all_inv(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..(n - 1))
        .flat_map(move |i| ((i + 1)..n).map(move |j| (i, j)))
        .filter(move |inv| inv != &(0, n - 1))
}

pub fn annealing(
    points: &Points,
    start: &mut [usize],
    start_temperature: f32,
    cooling: f32,
    epoch_samples: usize,
    max_stagnation: usize,
    max_epochs: usize,
) {
    let x = start;
    let n = x.len();
    let mut x_best = x.to_vec();

    let mut x_weight = measure_perm(points, x);
    let mut x_best_weight = x_weight;

    let mut temp = start_temperature;
    let mut last_improvement = 0;

    let dist = Uniform::new(0, n);

    for _ in 0..max_epochs {
        for _ in 0..epoch_samples {
            let y = gen_inv(&dist, n);
            let diff = inv_diff(points, x, y);
            if diff >= 0 {
                let p = (-diff as f32 / temp).exp();
                let rand = rand::random::<f32>();
                if rand > p {
                    continue;
                }
            }
            invert(x, y);
            x_weight = (x_weight as i32 + diff) as u32;

            if x_weight < x_best_weight {
                x_best.clone_from_slice(x);
                x_best_weight = x_weight;
                last_improvement = 0;
            }
        }
        
        temp *= cooling;
        last_improvement += 1;

        if last_improvement > max_stagnation {
            break;
        }
    }

    x.clone_from_slice(&x_best);
}

pub fn gen_inv(dist: &Uniform<usize>, n: usize) -> (usize, usize) {
    loop {
        let x = dist.sample(&mut thread_rng());
        let y = dist.sample(&mut thread_rng());
        if x != y && (x, y) != (0, n - 1) && (x, y) != (n - 1, 0) {
            return (x.min(y), x.max(y));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mst::{dfs_cycle, measure_mst, measure_perm, mst},
        parsing::parse_problem_from_str,
        points::Point,
    };

    use super::*;

    #[test]
    fn inv_diff_test() {
        let points = Points::from_points(vec![
            Point::new(0, 0),
            Point::new(2, 2),
            Point::new(0, 2),
            Point::new(2, 0),
        ]);
        let perm = &mut [3, 2, 1, 0];

        dbg!(measure_perm(&points, perm));

        dbg!(inv_diff(&points, perm, (3, 0)));

        invert(perm, (0, 3));

        dbg!(measure_perm(&points, perm));

        dbg!(inv_diff(&points, perm, (3, 0)));
    }

    #[test]
    fn all_inv_test() {
        for inv in all_inv(4) {
            dbg!(inv);
        }
    }

    #[test]
    fn invert_test() {
        let s = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        invert(s, (0, 9));
        dbg!(s);
    }

    #[test]
    fn xqf131_test() {
        let points = parse_problem_from_str(include_str!("./../../vlsi/xqf131.tsp")).unwrap();

        let mut perm = (0..points.list.len()).collect::<Vec<_>>();
        perm.shuffle(&mut thread_rng());
        let length = measure_perm(&points, &perm);

        annealing(&points, &mut perm, 200.0, 0.98, 1000, 100, usize::MAX);

        let opt_length = measure_perm(&points, &perm);

        dbg!(length, opt_length);
    }
}
