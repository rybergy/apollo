use crate::{
    common::*,
    othello::{Board, Othello},
};
use clap::ArgMatches;

pub fn main(matches: &ArgMatches) -> Result<(), String> {
    let a1_arg = matches.value_of("algorithm1").unwrap();
    let a2_arg = matches.value_of("algorithm2").unwrap();

    println!("{}: black", a1_arg);
    println!("{}: white", a2_arg);

    let (mut a1, d1) = extract_algorithm_depth(a1_arg)?;
    let (mut a2, d2) = extract_algorithm_depth(a2_arg)?;

    let mut game = Othello::new(Board::default());

    play(&mut game, &mut a1, d1, &mut a2, d2);

    println!("{}", game.board());

    Ok(())
}
