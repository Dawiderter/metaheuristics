use tsp::experiments::perform_and_save_test_for_graph;

pub fn main() {
    let names = [
        "xqf131", "xqg237", "pma343", "pka379", "bcl380", "pbl395", "pbk411", "pbn423", "pbm436",
        "xql662",
    ];
    let optimal_lengths = [
        564, 1019, 1368, 1332, 1621, 1281, 1343, 1365, 1443, 2513
    ];

    for (&name, &optimal) in names.iter().zip(optimal_lengths.iter()) {
        perform_and_save_test_for_graph(name);
        println!("Optimal: {optimal}")
    }
}
