use tabu::exp::experiment_one;

pub fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism().unwrap().get() - 1)
        .build_global()
        .unwrap();

    let names = [
        // "xqf131", "xqg237", "pma343", "pka379", "bcl380", "pbl395", "pbk411", "pbn423", "pbm436","xql662", 
        "xit1083", "icw1483", 
        "djc1785", "dcb2086", "pds2566",
    ];
    let optimal_lengths = [
        // Some(564),
        // Some(1019),
        // Some(1368),
        // Some(1332),
        // Some(1621),
        // Some(1281),
        // Some(1343),
        // Some(1365),
        // Some(1443),
        // Some(2513),
        Some(3558),
        Some(4416),
        Some(6115),
        Some(6600),
        None,
    ];

    for (&name, &optimal) in names.iter().zip(optimal_lengths.iter()) {
        experiment_one(name, optimal);
    }
}
