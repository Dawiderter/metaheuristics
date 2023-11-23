use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use crate::{
    mst::{dfs_cycle, measure_cycle, measure_mst, mst, random_cycle},
    parsing::parse_problem_from_str,
};

pub fn perform_and_save_test_for_graph(test_name: &str) {
    let text = read_to_string(format!("../vlsi/{}.tsp", test_name)).unwrap();
    let points = parse_problem_from_str(&text).unwrap();

    println!("========================================");
    //println!("Opened file {test_name}");

    let mst = mst(&points);
    let mst_size = measure_mst(&mst);
    //println!("MST length: {mst_size}");

    let cycle = dfs_cycle(&mst);
    let cycle_size = measure_cycle(&points, &cycle);
    //println!("MST TSP cycle length: {cycle_size}");

    let mut cycle_result_file =
        BufWriter::new(File::create(format!("./results/{test_name}")).unwrap());
    for &v in &cycle {
        let point = points.list[v];
        writeln!(&mut cycle_result_file, "{} {} {}", v, point.x, point.y).unwrap();
    }

    // let mut points_file = BufWriter::new(File::create(format!("./results/{test_name}.points")).unwrap());
    // for &point in &points.list {
    //     writeln!(&mut points_file, "{} {}", point.x, point.y).unwrap();
    // }

    let mut mst_result_file =
        BufWriter::new(File::create(format!("./results/{test_name}.mst")).unwrap());
    for (v, adj) in mst.adj.iter().enumerate() {
        for &(u, _) in adj {
            let point_v = points.list[v];
            let point_u = points.list[u];
            writeln!(
                &mut mst_result_file,
                "{} {} {} {}",
                point_v.x, point_v.y, point_u.x, point_u.y
            )
            .unwrap();
        }
    }

    let random_sizes = (0..1000)
        .map(|_| measure_cycle(&points, &random_cycle(points.list.len())))
        .collect::<Vec<_>>();

    let avg_min_10s = random_sizes
        .chunks(10)
        .map(|chunk| chunk.iter().min().unwrap())
        .sum::<u32>() as f64
        / 100.0;
    //println!("Avg of min from groups of 10: {avg_min_10s}");

    let avg_min_50s = random_sizes
        .chunks(50)
        .map(|chunk| chunk.iter().min().unwrap())
        .sum::<u32>() as f64
        / 20.0;
    //println!("Avg of min from groups of 50: {avg_min_50s}");

    let &global_min = random_sizes.iter().min().unwrap();
    //println!("Min of all: {global_min}");

    println!("|{test_name}|{mst_size}|{cycle_size}|{avg_min_10s}|{avg_min_50s}|{global_min}|")
}
