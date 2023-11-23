use crate::points::Points;

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
}

pub fn local_search(points: &Points, start: &mut [usize]) -> u32 {
    let x = start;
    let n = x.len();

    let mut count = 0;

    for _ in 0..10_000 {
        let min_inv = all_inv(n)
            .min_by_key(|&inv| inv_diff(points, x, inv))
            .unwrap();

        if inv_diff(points, x, min_inv) >= 0 {
            break;
        } else {
            count += 1;
            invert(x, min_inv);
        }
    }

    if count >= 10_000 {
        println!("Too long loop")
    }

    count
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

        let mst = mst(&points);
        let mst_length = measure_mst(&mst);
        let mut perm = dfs_cycle(&mst, 20);
        let length = measure_perm(&points, &perm);

        let count = local_search(&points, &mut perm);

        let opt_length = measure_perm(&points, &perm);

        dbg!(mst_length, length, opt_length, count);
    }
}
