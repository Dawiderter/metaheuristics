use std::fs::read_to_string;

use tabu::{
    exp::{tabu_test_param, tabu_test_param_mst, tabu_test_param_mst_rand, a_test_param},
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

    let test_points = names.iter().map(|name| {
        let text = read_to_string(format!("../vlsi/{}.tsp", name)).unwrap();
        parse_problem_from_str(&text).unwrap()
    }).collect::<Vec<_>>();

    let optimal_lens = [564, 1019, 1368, 1332, 1621, 1281, 1343, 1365, 1443, 2513];

    let default_epoch_samples = 1.0;
    let default_max_stagnation = 1.0;
    let default_max_epochs = usize::MAX;
    let default_temp = 1.0;

    let coolings = [0.8,0.85, 0.9, 0.95, 0.98];

    for cooling in coolings {
        println!("==========");
        println!("Cooling: {cooling}");
        let mut time_sum = 0.0;
        let mut opt_sum = 0.0;

        for (points, &opt) in test_points.iter().zip(optimal_lens.iter()) {

            let n = points.list.len();
    
            let (avg_time, avg_len) = a_test_param(
                points,
                default_temp * n as f32,
                cooling,
                (default_epoch_samples * n as f32) as usize,
                (default_max_stagnation * n as f32) as usize,
                default_max_epochs,
                50,
            );
    
            time_sum += avg_time;
            opt_sum += avg_len / (opt as f32);
        }
        println!("Avg time: {}", time_sum / test_points.len() as f32);
        println!("Opt ratio: {}", opt_sum / test_points.len() as f32);
    }

}
