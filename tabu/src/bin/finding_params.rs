use std::fs::read_to_string;

use tabu::{
    exp::{tabu_test_param, tabu_test_param_mst, tabu_test_param_mst_rand},
    parsing::parse_problem_from_str,
};

pub fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism().unwrap().get() - 1)
        .build_global()
        .unwrap();
    let names = [
        "xqf131", "xqg237", "pma343", "pka379", "bcl380", "pbl395", "pbk411", "pbn423", "pbm436",
        "xql662",
    ];

    // let test_points = names.iter().map(|name| {
    //     let text = read_to_string(format!("../vlsi/{}.tsp", name)).unwrap();
    //     parse_problem_from_str(&text).unwrap()
    // }).collect::<Vec<_>>();

    let optimal_lens = [564, 1019, 1368, 1332, 1621, 1281, 1343, 1365, 1443, 2513];

    let default_max_list_len = 100;
    let default_max_iter = usize::MAX;
    let default_max_stagnation = default_max_list_len;

    //let max_list_lens = [7, 14, 21, 49, 98, 196, 343, 686, 1372, 2401];

    for (name, opt) in names.into_iter().zip(optimal_lens.into_iter()) {

        let text = read_to_string(format!("../vlsi/{}.tsp", name)).unwrap();
        let points = parse_problem_from_str(&text).unwrap();

        println!("==========");
        println!("Test: {name}");

        let (avg_time, avg_len) = tabu_test_param_mst_rand(
            &points,
            default_max_list_len,
            default_max_iter,
            points.list.len()*10,
            100,
        );

        println!("Avg time: {avg_time}");
        println!("Opt ratio: {}", avg_len / (opt as f32));
    }
}
