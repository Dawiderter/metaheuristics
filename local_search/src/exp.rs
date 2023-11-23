use std::{
    fs::{File, read_to_string},
    io::{BufWriter, Write},
    iter,
    time::Duration, sync::Mutex,
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{distributions::Uniform, prelude::*};
use rayon::prelude::*;

use crate::{
    local_search::local_search,
    mst::{dfs_cycle, measure_perm, mst, measure_mst},
    parsing::parse_problem_from_str,
    points::Points,
};

pub fn write_perm(test_name: &str, points: &Points, perm: &[usize]) {
    let mut perm_result_file =
        BufWriter::new(File::create(format!("./results/{test_name}")).unwrap());
    for &v in perm.iter().chain(iter::once(perm.first().unwrap())) {
        let point = points.list[v];
        writeln!(&mut perm_result_file, "{} {} {}", v, point.x, point.y).unwrap();
    }
}

pub fn experiment_one(test_name: &str, optimal: Option<usize>) {
    let text = read_to_string(format!("../vlsi/{}.tsp", test_name)).unwrap();
    let points = parse_problem_from_str(&text).unwrap();
    let n = points.list.len();
    let sample_size = ((n as f32).sqrt()) as usize;
    let mst = mst(&points);
    let mst_length = measure_mst(&mst);

    println!("==================");
    let progress_info = MultiProgress::new();
    let progress_bar = ProgressBar::new(sample_size as u64)
        .with_style(ProgressStyle::with_template("Progress: {bar:20.cyan/blue} {pos:>7}/{len:7}").unwrap());
    let time_bar = ProgressBar::new_spinner()
        .with_style(ProgressStyle::with_template("Elapsed time: [{elapsed_precise}] {spinner}").unwrap());
    progress_info.println(format!("Beginning test {}", test_name)).unwrap();

    let progress_bar = progress_info.add(progress_bar);
    let time_bar = progress_info.add(time_bar);

    progress_bar.tick();
    time_bar.enable_steady_tick(Duration::from_secs(1));

    let distribution = Uniform::new(0, n);

    // count_sum, length_sum, min_length, min_perm
    let stats = Mutex::new((0, 0, u32::MAX, Vec::new()));

    (0..sample_size)
        .into_par_iter()
        .for_each(|_| {
            let mut perm = dfs_cycle(&mst, distribution.sample(&mut thread_rng()));
            let count = local_search(&points, &mut perm);
            let opt_length = measure_perm(&points, &perm);

            progress_bar.inc(1);

            let mut stats = stats.lock().unwrap();

            if opt_length < stats.2 {
                stats.2 = opt_length;
                stats.3 = perm;
            }

            stats.0 += count;
            stats.1 += opt_length
        });

    let (count_sum, length_sum, min_length, min_perm) = stats.into_inner().unwrap();

    write_perm(test_name, &points, &min_perm);
    progress_bar.finish();
    time_bar.finish();
    if let Some(optimal) = optimal {
        println!("Optimal length: {}", optimal);
    }
    println!("MST length: {}", mst_length);
    println!("Minimal found length: {}", min_length);
    println!("Avg improvements: {}",count_sum as f32 / sample_size as f32);
    println!("Avg length: {}", length_sum as f32 / sample_size as f32);
}

pub fn experiment_two(test_name: &str, optimal: Option<usize>) {
    let text = read_to_string(format!("../vlsi/{}.tsp", test_name)).unwrap();
    let points = parse_problem_from_str(&text).unwrap();
    let n = points.list.len();
    let sample_size = if n >= 1000 { 100 } else { n };

    println!("==================");
    let progress_info = MultiProgress::new();
    let progress_bar = ProgressBar::new(sample_size as u64)
        .with_style(ProgressStyle::with_template("Progress: {bar:20.cyan/blue} {pos:>7}/{len:7}").unwrap());
    let time_bar = ProgressBar::new_spinner()
        .with_style(ProgressStyle::with_template("Elapsed time: [{elapsed_precise}] {spinner}").unwrap());
    progress_info.println(format!("Beginning test {}", test_name)).unwrap();

    let progress_bar = progress_info.add(progress_bar);
    let time_bar = progress_info.add(time_bar);

    progress_bar.tick();
    time_bar.enable_steady_tick(Duration::from_secs(1));

    // count_sum, length_sum, min_length, min_perm
    let stats = Mutex::new((0, 0, u32::MAX, Vec::new()));

    (0..sample_size)
        .into_par_iter()
        .for_each(|_| {
            let mut perm = (0..n).collect::<Vec<_>>();
            perm.shuffle(&mut thread_rng());
            let count = local_search(&points, &mut perm);
            let opt_length = measure_perm(&points, &perm);

            progress_bar.inc(1);

            let mut stats = stats.lock().unwrap();

            if opt_length < stats.2 {
                stats.2 = opt_length;
                stats.3 = perm;
            }

            stats.0 += count;
            stats.1 += opt_length
        });

    let (count_sum, length_sum, min_length, min_perm) = stats.into_inner().unwrap();

    write_perm(test_name, &points, &min_perm);
    progress_bar.finish();
    time_bar.finish();
    if let Some(optimal) = optimal {
        println!("Optimal length: {}", optimal);
    }
    println!("Minimal found length: {}", min_length);
    println!("Avg improvements: {}",count_sum as f32 / sample_size as f32);
    println!("Avg length: {}", length_sum as f32 / sample_size as f32);
}