use crate::{common::*, othello::Disc, solve::Search};
use clap::ArgMatches;
use rand::{thread_rng, Rng};
use std::time::SystemTime;

pub fn main(matches: &ArgMatches) -> Result<(), String> {
    let n = matches
        .value_of("number")
        .unwrap_or("50")
        .parse::<usize>()
        .map_err(|e| e.to_string())?;

    let depth = matches
        .value_of("depth")
        .unwrap_or("7")
        .parse::<usize>()
        .map_err(|e| e.to_string())?;

    let lower = matches
        .value_of("lower")
        .unwrap_or("5")
        .parse::<usize>()
        .map_err(|e| e.to_string())?;

    let upper = matches
        .value_of("upper")
        .unwrap_or("40")
        .parse::<usize>()
        .map_err(|e| e.to_string())?;

    let mut algs = Vec::new();
    let alg_strs = matches.values_of("algorithms").unwrap();

    print!("depth,");
    for alg_str in alg_strs.clone() {
        print!("generated - {},", alg_str);
        algs.push(extract_search_algorithm(alg_str)?);
    }
    for alg_str in alg_strs.clone() {
        print!("expanded - {},", alg_str);
    }
    for alg_str in alg_strs.clone() {
        print!("time - {},", alg_str);
    }

    eprintln!(
        "Benchmarking performance of algorithms {:?}",
        alg_strs.collect::<Vec<_>>()
    );
    eprintln!(" number of trials: {}", n);
    eprintln!(" maximum depth: {}", depth);
    eprintln!(" lower bound on random moves: {}", lower);
    eprintln!(" upper bound on random moves: {}", upper);

    println!("");

    performance(&mut algs, n, depth, lower, upper);

    Ok(())
}

fn performance(
    algs: &mut Vec<Box<dyn Search>>,
    n: usize,
    max_depth: usize,
    lower: usize,
    upper: usize,
) {
    let mut rng = thread_rng();
    let mut depths = Vec::new();
    for depth in 1..(max_depth + 1) {
        print!("{},", depth);

        let mut expandeds = (0..algs.len()).map(|_| 0.).collect::<Vec<_>>();
        let mut generateds = (0..algs.len()).map(|_| 0.).collect::<Vec<_>>();
        let mut times = (0..algs.len()).map(|_| 0.).collect::<Vec<_>>();
        for _ in 0..n {
            let num_moves = lower + rng.gen::<usize>() % (upper - lower);
            let game = random_game(num_moves);

            for (i, alg) in algs.iter_mut().enumerate() {
                let start = SystemTime::now();
                alg.search(&game, Disc::Black, depth);

                let expanded = alg.nodes_expanded();
                expandeds[i] += expanded as f64;

                let generated = alg.nodes_generated();
                generateds[i] += generated as f64;

                let elapsed = start.elapsed().unwrap();
                let ms = elapsed.subsec_millis();
                times[i] += ms as f64;
            }
        }

        for &sum in generateds.iter() {
            let generated = sum / n as f64;
            print!("{},", generated);
        }

        for &sum in expandeds.iter() {
            let expanded = sum / n as f64;
            print!("{},", expanded);
        }

        for &sum in times.iter() {
            let time = sum / n as f64;
            print!("{},", time);
        }

        println!("");

        depths.push((expandeds, times));
    }
}
