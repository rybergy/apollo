use crate::othello::{Board, Disc, Othello};
use crate::solve::*;
use rand::{thread_rng, Rng};

fn extract_heuristic_name(s: &str) -> Result<Box<dyn Heuristic>, String> {
    let heuristic: Box<dyn Heuristic> = match s {
        "0" => HZero::new(),
        "random" => HRandom::new(),
        "unit" => HUnit::new(),
        "weight" => HWeighted::new(),
        "mobility" => HMobility::new(),
        "weight-mobility" => HWeightedMobility::new(),
        other => return Err(format!("Unknown heuristic function '{}'!", other)),
    };

    Ok(heuristic)
}

fn extract_search_name(s: &str, heuristic: Box<dyn Heuristic>) -> Result<Box<dyn Search>, String> {
    let search: Box<dyn Search> = match s {
        "mini" => Minimax::new(heuristic),
        "ab" => AlphaBeta::new(heuristic),
        "ab-order" => AlphaBetaOrdering::new(heuristic),
        "ab-order-unit" => AlphaBetaOrderingUnit::new(heuristic),
        other => return Err(format!("Unknown search algorithm name '{}'!", other)),
    };

    Ok(search)
}

pub fn extract_algorithm_depth(s: &str) -> Result<(Box<dyn Search>, usize), String> {
    let mut parts = s.split(":");
    let (search_name, heuristic_name, depth_str) = match (parts.next(), parts.next(), parts.next())
    {
        (Some(search), Some(heuristic), Some(depth)) => (search, heuristic, depth),
        _ => return Err(format!("Invalid depth-suffixed algorithm string '{}'!", s)),
    };

    let heuristic = extract_heuristic_name(heuristic_name)?;
    let alg = extract_search_name(search_name, heuristic)?;

    let depth = depth_str.parse::<usize>().map_err(|e| e.to_string())?;

    Ok((alg, depth))
}

pub fn extract_search_algorithm(s: &str) -> Result<Box<dyn Search>, String> {
    let mut parts = s.split(":");
    let (search_name, heuristic_name) = match (parts.next(), parts.next()) {
        (Some(search), Some(heuristic)) => (search, heuristic),
        _ => {
            return Err(format!(
                "Invalid non-depth-suffixed algorithm string '{}'!",
                s
            ))
        }
    };

    let heuristic = extract_heuristic_name(heuristic_name)?;
    let alg = extract_search_name(search_name, heuristic)?;

    Ok(alg)
}

pub fn play(
    game: &mut Othello,
    a1: &mut Box<dyn Search>,
    d1: usize,
    a2: &mut Box<dyn Search>,
    d2: usize,
) {
    let p1 = Disc::Black;
    let p2 = Disc::White;

    while game.any_valid_moves() {
        turn(game, a1, d1, p1);
        turn(game, a2, d2, p2);
    }
}

fn turn(game: &mut Othello, alg: &mut Box<dyn Search>, depth: usize, player: Disc) {
    if let (Some(mv), _) = alg.search(game, player, depth) {
        game.place(mv, player);
    }
}

pub fn random_game(moves: usize) -> Othello {
    let game = Othello::new(Board::default());

    let mut random = Minimax::new(HRandom::new());
    let mut player = Disc::Black;

    for _ in 0..moves {
        random.search(&game, player, 1);
        player = player.opponent();
    }

    game
}

pub fn winrate(
    a1: &mut Box<dyn Search>,
    d1: usize,
    a2: &mut Box<dyn Search>,
    d2: usize,
    n: usize,
    lower: usize,
    upper: usize,
) -> f64 {
    let mut rng = thread_rng();

    let mut a1wins = 0;
    let mut a1_black = true;

    for _ in 0..n {
        let num_moves = lower + rng.gen::<usize>() % (upper - lower);
        let mut game = random_game(num_moves);

        if a1_black {
            play(&mut game, a1, d1, a2, d2);
        } else {
            play(&mut game, a2, d2, a1, d1);
        }

        let black_won = game.winner().unwrap_or(Disc::White) == Disc::Black;

        if (black_won && a1_black) || (!black_won && !a1_black) {
            a1wins += 1;
        }

        a1_black = !a1_black;
    }

    a1wins as f64 / n as f64
}
