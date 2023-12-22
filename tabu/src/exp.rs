use std::{
    fs::read_to_string,
    sync::Mutex,
    time::{Duration, Instant},
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

use crate::{
    mst::{measure_perm, mst, dfs_cycle}, parsing::parse_problem_from_str, points::Points, tabu::{random_tabu, tabu}, annihiling::annealing,
};

pub fn experiment_one(test_name: &str, optimal: Option<usize>) {
    let text = read_to_string(format!("../vlsi/{}.tsp", test_name)).unwrap();
    let points = parse_problem_from_str(&text).unwrap();
    let n = points.list.len();
    let sample_size = 100;
    let mst = mst(&points);

    println!("==================");
    let progress_info = MultiProgress::new();
    let progress_bar = ProgressBar::new(sample_size as u64).with_style(
        ProgressStyle::with_template("Progress: {bar:20.cyan/blue} {pos:>7}/{len:7}").unwrap(),
    );
    let time_bar = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template("Elapsed time: [{elapsed_precise}] {spinner}").unwrap(),
    );
    progress_info
        .println(format!("Beginning test {}", test_name))
        .unwrap();

    let progress_bar = progress_info.add(progress_bar);
    let time_bar = progress_info.add(time_bar);

    progress_bar.tick();
    time_bar.enable_steady_tick(Duration::from_secs(1));

    // count_sum, length_sum, min_length, min_perm
    let stats = Mutex::new((0, 0, u32::MAX, Vec::new()));

    (0..sample_size).into_par_iter().for_each(|_| {
        let mut perm = dfs_cycle(&mst, thread_rng().gen_range(0..n));
        let count = tabu(&points, &mut perm, 50, usize::MAX, 50);
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

    let (count_sum, length_sum, min_length, _) = stats.into_inner().unwrap();

    progress_bar.finish();
    time_bar.finish();
    if let Some(optimal) = optimal {
        println!("Optimal length: {}", optimal);
    }
    println!("Minimal found length: {}", min_length);
    println!(
        "Avg improvements: {}",
        count_sum as f32 / sample_size as f32
    );
    println!("Avg length: {}", length_sum as f32 / sample_size as f32);
}

pub fn experiment_two(test_name: &str, optimal: Option<usize>) {
    let text = read_to_string(format!("../vlsi/{}.tsp", test_name)).unwrap();
    let points = parse_problem_from_str(&text).unwrap();
    let n = points.list.len();
    let sample_size = 100;

    println!("==================");
    let progress_info = MultiProgress::new();
    let progress_bar = ProgressBar::new(sample_size as u64).with_style(
        ProgressStyle::with_template("Progress: {bar:20.cyan/blue} {pos:>7}/{len:7}").unwrap(),
    );
    let time_bar = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template("Elapsed time: [{elapsed_precise}] {spinner}").unwrap(),
    );
    progress_info
        .println(format!("Beginning test {}", test_name))
        .unwrap();

    let progress_bar = progress_info.add(progress_bar);
    let time_bar = progress_info.add(time_bar);

    progress_bar.tick();
    time_bar.enable_steady_tick(Duration::from_secs(1));

    // count_sum, length_sum, min_length, min_perm
    let stats = Mutex::new((0, 0, u32::MAX, Vec::new()));

    (0..sample_size).into_par_iter().for_each(|_| {
        let mut perm = (0..n).collect::<Vec<_>>();
        perm.shuffle(&mut thread_rng());
        annealing(&points, &mut perm, 
            1.0 * n as f32,
            0.98,
            (5.0 * n as f32) as usize,
            (2.0 * n as f32) as usize,
            usize::MAX
        );
        let opt_length = measure_perm(&points, &perm);

        progress_bar.inc(1);

        let mut stats = stats.lock().unwrap();

        if opt_length < stats.2 {
            stats.2 = opt_length;
            stats.3 = perm;
        }

        stats.1 += opt_length
    });

    let (_, length_sum, min_length, _) = stats.into_inner().unwrap();

    progress_bar.finish();
    time_bar.finish();
    if let Some(optimal) = optimal {
        println!("Optimal length: {}", optimal);
    }
    println!("Minimal found length: {}", min_length);
    println!("Avg length: {}", length_sum as f32 / sample_size as f32);
}

/// avg_time, avg_len
pub fn tabu_test_param(
    points: &Points,
    max_list_len: usize,
    max_iter: usize,
    max_stagnations: usize,
    sample_size: usize,
) -> (f32, f32) {
    let n = points.list.len();

    // count_sum, length_sum, min_length, min_perm
    let stats = Mutex::new((0, 0, u32::MAX, Vec::new()));

    let progress_info = MultiProgress::new();
    let progress_bar = ProgressBar::new(sample_size as u64).with_style(
        ProgressStyle::with_template("Progress: {bar:20.cyan/blue} {pos:>7}/{len:7}").unwrap(),
    );
    let time_bar = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template("Elapsed time: [{elapsed_precise}] {spinner}").unwrap(),
    );
    let progress_bar = progress_info.add(progress_bar);
    let time_bar = progress_info.add(time_bar);
    progress_bar.tick();
    time_bar.enable_steady_tick(Duration::from_secs(1));

    let time_start = Instant::now();

    (0..sample_size).into_par_iter().for_each(|_| {
        let mut perm = (0..n).collect::<Vec<_>>();
        perm.shuffle(&mut thread_rng());
        let count = tabu(points, &mut perm, max_list_len, max_iter, max_stagnations);
        let opt_length = measure_perm(points, &perm);

        let mut stats = stats.lock().unwrap();

        progress_bar.inc(1);

        if opt_length < stats.2 {
            stats.2 = opt_length;
            stats.3 = perm;
        }

        stats.0 += count;
        stats.1 += opt_length
    });
    
    progress_bar.finish();
    time_bar.finish();

    let avg_time = (Instant::now() - time_start).as_secs_f32() / sample_size as f32;

    let (_, length_sum, _, _) = stats.into_inner().unwrap();

    let avg_len = length_sum as f32 / sample_size as f32;

    (avg_time, avg_len)
}

/// avg_time, avg_len
pub fn tabu_test_param_mst(
    points: &Points,
    max_list_len: usize,
    max_iter: usize,
    max_stagnations: usize,
    sample_size: usize,
) -> (f32, f32) {
    let n = points.list.len();
    let mst = mst(points);

    // count_sum, length_sum, min_length, min_perm
    let stats = Mutex::new((0, 0, u32::MAX, Vec::new()));

    let progress_info = MultiProgress::new();
    let progress_bar = ProgressBar::new(sample_size as u64).with_style(
        ProgressStyle::with_template("Progress: {bar:20.cyan/blue} {pos:>7}/{len:7}").unwrap(),
    );
    let time_bar = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template("Elapsed time: [{elapsed_precise}] {spinner}").unwrap(),
    );
    let progress_bar = progress_info.add(progress_bar);
    let time_bar = progress_info.add(time_bar);
    progress_bar.tick();
    time_bar.enable_steady_tick(Duration::from_secs(1));

    let time_start = Instant::now();

    (0..sample_size).into_par_iter().for_each(|_| {
        let mut perm = dfs_cycle(&mst, thread_rng().gen_range(0..n));
        let count = tabu(points, &mut perm, max_list_len, max_iter, max_stagnations);

        progress_bar.inc(1);

        let opt_length = measure_perm(points, &perm);

        let mut stats = stats.lock().unwrap();

        if opt_length < stats.2 {
            stats.2 = opt_length;
            stats.3 = perm;
        }

        stats.0 += count;
        stats.1 += opt_length
    });

    progress_bar.finish();
    time_bar.finish();

    let avg_time = (Instant::now() - time_start).as_secs_f32() / sample_size as f32;

    let (_, length_sum, _, _) = stats.into_inner().unwrap();

    let avg_len = length_sum as f32 / sample_size as f32;

    (avg_time, avg_len)
}

/// avg_time, avg_len
pub fn tabu_test_param_mst_rand(
    points: &Points,
    max_list_len: usize,
    max_iter: usize,
    max_stagnations: usize,
    sample_size: usize,
) -> (f32, f32) {
    let n = points.list.len();
    let mst = mst(points);

    // count_sum, length_sum, min_length, min_perm
    let stats = Mutex::new((0, 0, u32::MAX, Vec::new()));

    let progress_info = MultiProgress::new();
    let progress_bar = ProgressBar::new(sample_size as u64).with_style(
        ProgressStyle::with_template("Progress: {bar:20.cyan/blue} {pos:>7}/{len:7}").unwrap(),
    );
    let time_bar = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template("Elapsed time: [{elapsed_precise}] {spinner}").unwrap(),
    );
    let progress_bar = progress_info.add(progress_bar);
    let time_bar = progress_info.add(time_bar);
    progress_bar.tick();
    time_bar.enable_steady_tick(Duration::from_secs(1));

    let time_start = Instant::now();

    (0..sample_size).into_par_iter().for_each(|_| {
        let mut perm = dfs_cycle(&mst, thread_rng().gen_range(0..n));
        let count = random_tabu(points, &mut perm, max_list_len, max_iter, max_stagnations);

        progress_bar.inc(1);

        let opt_length = measure_perm(points, &perm);

        let mut stats = stats.lock().unwrap();

        if opt_length < stats.2 {
            stats.2 = opt_length;
            stats.3 = perm;
        }

        stats.0 += count;
        stats.1 += opt_length
    });

    progress_bar.finish();
    time_bar.finish();

    let avg_time = (Instant::now() - time_start).as_secs_f32() / sample_size as f32;

    let (_, length_sum, _, _) = stats.into_inner().unwrap();

    let avg_len = length_sum as f32 / sample_size as f32;

    (avg_time, avg_len)
}

/// avg_time, avg_len
pub fn a_test_param(
    points: &Points,
    start_temperature: f32,
    cooling: f32,
    epoch_samples: usize,
    max_stagnation: usize,
    max_epochs: usize,
    sample_size: usize,
) -> (f32, f32) {
    let n = points.list.len();

    // count_sum, length_sum, min_length, min_perm
    let stats = Mutex::new((0, 0, u32::MAX, Vec::new()));

    let time_start = Instant::now();

    (0..sample_size).into_par_iter().for_each(|_| {
        let mut perm = (0..n).collect::<Vec<_>>();
        perm.shuffle(&mut thread_rng());

        annealing(points, &mut perm, start_temperature, cooling, epoch_samples, max_stagnation, max_epochs);

        let opt_length = measure_perm(points, &perm);

        let mut stats = stats.lock().unwrap();

        if opt_length < stats.2 {
            stats.2 = opt_length;
            stats.3 = perm;
        }

        stats.1 += opt_length
    });

    let avg_time = (Instant::now() - time_start).as_secs_f32() / sample_size as f32;

    let (_, length_sum, _, _) = stats.into_inner().unwrap();

    let avg_len = length_sum as f32 / sample_size as f32;

    (avg_time, avg_len)
}