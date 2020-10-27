#[macro_use]
extern crate clap;

mod benchmark;
mod common;
pub mod othello;
mod play;
mod sim;
pub mod solve;

fn main() -> Result<(), String> {
    let matches = clap_app!(Apollo =>
        (version: "v0.1.0")
        (author: "Ryan Bergman <rybergy@gmail.com>")
        (about: "An Othello player/benchmarker.")
        (@setting SubcommandRequiredElseHelp)
        (@setting ArgRequiredElseHelp)
        (@setting ColoredHelp)
        (@subcommand benchmark =>
            (about: "Benchmarking utilities which output CSV.")
            (version: "v0.1.0")
            (author: "Ryan Bergman <rybergy@gmail.com>")
            (@setting SubcommandRequiredElseHelp)
            (@setting ColoredHelp)
            (@subcommand winrate =>
                (about: "Benchmarks the winrate of multiple algorithms playing against each other.")
                (version: "v0.1.0")
                (author: "Ryan Bergman <rybergy@gmail.com>")
                (@setting ColoredHelp)
                (@arg number: -n --number +takes_value "The number of games to play (default 100).")
                (@arg lower: -l --lower +takes_value "The lower bound of random moves to perform (default 5)")
                (@arg upper: -u --upper +takes_value "The upper bound of random moves to perform (default 40)")
                (@arg algorithms: ... +required "The algorithms to use (depth-suffixed).")
            )
            (@subcommand performance =>
                (about: "Benchmarks the performance of multiple algorithms and outputs to CSV.")
                (version: "v0.1.0")
                (author: "Ryan Bergman <rybergy@gmail.com>")
                (@setting ArgRequiredElseHelp)
                (@setting ColoredHelp)
                (@arg number: -n --number +takes_value "The number of tests to run (default 100).")
                (@arg depth: -d --depth +takes_value "The maximum depth limit (default 7).")
                (@arg lower: -l --lower +takes_value "The lower bound of random moves to perform (default 5)")
                (@arg upper: -u --upper +takes_value "The upper bound of random moves to perform (default 40)")
                (@arg algorithms: ... +required "All algorithms to benchmark (not depth-suffixed).")
            )
        )
        (@subcommand sim =>
            (about: "Simulates a single game of othello.")
            (version: "v0.1.0")
            (author: "Ryan Bergman <rybergy@gmail.com>")
            (@setting ArgRequiredElseHelp)
            (@setting ColoredHelp)
            (@arg algorithm1: +required +takes_value {is_algorithm_string} "The algorithm to use for player 1.")
            (@arg algorithm2: +required +takes_value {is_algorithm_string} "The algorithm to use for player 2.")
        )
        (@subcommand play =>
            (about: "Play a game of othello against the CPU. Use W/A/S/D to move the selection and press space to drop a piece.")
            (version: "v0.1.0")
            (author: "Ryan Bergman <rybergy@gmail.com>")
            (@setting ColoredHelp)
        )
    )
    .get_matches();

    if let Some(m) = matches.subcommand_matches("benchmark") {
        benchmark::main(m)?;
    } else if let Some(m) = matches.subcommand_matches("sim") {
        sim::main(m)?;
    } else if let Some(m) = matches.subcommand_matches("play") {
        play::main(m).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn is_algorithm_string(s: String) -> Result<(), String> {
    common::extract_algorithm_depth(&*s)?;
    Ok(())
}
