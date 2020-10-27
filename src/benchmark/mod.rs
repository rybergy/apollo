mod performance;
mod winrate;

use clap::ArgMatches;

pub fn main(matches: &ArgMatches) -> Result<(), String> {
    if let Some(m) = matches.subcommand_matches("performance") {
        performance::main(m)?;
    } else if let Some(m) = matches.subcommand_matches("winrate") {
        winrate::main(m)?;
    }
    Ok(())
}
