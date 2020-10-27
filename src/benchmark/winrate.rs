use crate::{common::*, solve::Search};
use clap::ArgMatches;

pub fn main(matches: &ArgMatches) -> Result<(), String> {
    let n = matches
        .value_of("number")
        .unwrap_or("100")
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

    let mut names = Vec::new();
    let mut algs = Vec::new();

    for alg_str in matches.values_of("algorithms").unwrap() {
        let (alg, specified_depth) = extract_algorithm_depth(alg_str)?;
        names.push(alg_str);
        algs.push((alg, specified_depth));
    }

    eprintln!("Benchmarking winrate of algorithms {:?}", names);
    eprintln!(" number of trials: {}", n);
    eprintln!(" lower bound on random moves: {}", lower);
    eprintln!(" upper bound on random moves: {}", upper);

    print!(",");
    for name in names.iter() {
        print!("{},", name);
    }
    println!("");

    let winrates = winrate_all(&mut algs, n, lower, upper);

    for (index, row) in winrates.iter().enumerate() {
        print!("{},", names[index]);
        for winrate in row.iter() {
            match winrate {
                Some(rate) => print!("{},", rate),
                None => print!(","),
            }
        }
        println!("");
    }

    Ok(())
}

fn winrate_all(
    algs: &mut Vec<(Box<dyn Search>, usize)>,
    n: usize,
    lower: usize,
    upper: usize,
) -> Vec<Vec<Option<f64>>> {
    let mut winrates: Vec<Vec<Option<f64>>> = (0..algs.len())
        .map(|_| (0..algs.len()).map(|_| None).collect())
        .collect();

    for i in 0..algs.len() {
        for j in (i + 1)..algs.len() {
            let (p1, p2) = algs.split_at_mut(j);
            let (a1, d1) = &mut p1[i];
            let (a2, d2) = &mut p2[0];
            let a1_winrate = winrate(a1, *d1, a2, *d2, n, lower, upper);

            winrates[i][j] = Some(a1_winrate);
            winrates[j][i] = Some(1. - a1_winrate);
        }
    }

    winrates
}
