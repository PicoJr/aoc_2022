use log::debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Score = u64;

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Lost,
    Draw,
    Won,
}

#[derive(Debug)]
struct RoundChallenge1 {
    first: Shape,
    second: Shape,
}

#[derive(Debug)]
struct RoundChallenge2 {
    first: Shape,
    second: Outcome,
}

fn shape_from_str_hopefuly(value: &str) -> anyhow::Result<Shape> {
    match value {
        "A" => Ok(Shape::Rock),
        "B" => Ok(Shape::Paper),
        "C" => Ok(Shape::Scissors),
        "X" => Ok(Shape::Rock),
        "Y" => Ok(Shape::Paper),
        "Z" => Ok(Shape::Scissors),
        _ => anyhow::bail!("unknown letter {}", value),
    }
}

fn outcome_from_str_hopefuly(value: &str) -> anyhow::Result<Outcome> {
    match value {
        "X" => Ok(Outcome::Lost),
        "Y" => Ok(Outcome::Draw),
        "Z" => Ok(Outcome::Won),
        _ => anyhow::bail!("unknown letter {}", value),
    }
}

fn outcome(player: &Shape, opponent: &Shape) -> Outcome {
    match (player, opponent) {
        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => Outcome::Won,
        _ => Outcome::Lost,
    }
}

fn shape_for_outcome(opponent_shape: &Shape, outcome: &Outcome) -> Shape {
    match (opponent_shape, outcome) {
        (Shape::Rock, Outcome::Lost) => Shape::Scissors,
        (Shape::Paper, Outcome::Lost) => Shape::Rock,
        (Shape::Scissors, Outcome::Lost) => Shape::Paper,
        (Shape::Rock, Outcome::Won) => Shape::Paper,
        (Shape::Paper, Outcome::Won) => Shape::Scissors,
        (Shape::Scissors, Outcome::Won) => Shape::Rock,
        (s, Outcome::Draw) => s.clone(),
    }
}

fn score_round(player: &Shape, opponent: &Shape) -> Score {
    let score_from_outcome = match outcome(player, opponent) {
        Outcome::Lost => 0,
        Outcome::Draw => 3,
        Outcome::Won => 6,
    };
    let score_from_shape = match player {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
    debug!(
        "{:?} vs {:?} -> {} + {} = {}",
        player,
        opponent,
        score_from_outcome,
        score_from_shape,
        score_from_outcome + score_from_shape
    );
    score_from_outcome + score_from_shape
}

fn parse_input_file_challenge_1(input_path: &Path) -> anyhow::Result<Vec<RoundChallenge1>> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let mut rounds: Vec<RoundChallenge1> = vec![];
    for line in buffer_reader.lines() {
        let line = line?;
        let letters: Vec<&str> = line.split(' ').collect();
        let first_letter = letters
            .first()
            .ok_or_else(|| anyhow::anyhow!("parsing error {}", line))?;
        let second_letter = letters
            .get(1)
            .ok_or_else(|| anyhow::anyhow!("parsing error {}", line))?;
        let first: Shape = shape_from_str_hopefuly(first_letter)?;
        let second: Shape = shape_from_str_hopefuly(second_letter)?;
        rounds.push(RoundChallenge1 { first, second })
    }
    Ok(rounds)
}

fn score_rounds(rounds: &[RoundChallenge1]) -> Score {
    // first letter is opponent, second letter is the player for which we compute the score
    rounds
        .iter()
        .map(|round| score_round(&round.second, &round.first))
        .sum()
}

pub(crate) fn solve_day_2_challenge_1(input_path: &Path) -> anyhow::Result<Score> {
    let rounds = parse_input_file_challenge_1(input_path)?;
    Ok(score_rounds(rounds.as_slice()))
}

fn parse_input_file_challenge_2(input_path: &Path) -> anyhow::Result<Vec<RoundChallenge2>> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let mut rounds: Vec<RoundChallenge2> = vec![];
    for line in buffer_reader.lines() {
        let line = line?;
        let letters: Vec<&str> = line.split(' ').collect();
        let first_letter = letters
            .first()
            .ok_or_else(|| anyhow::anyhow!("parsing error {}", line))?;
        let second_letter = letters
            .get(1)
            .ok_or_else(|| anyhow::anyhow!("parsing error {}", line))?;
        let first: Shape = shape_from_str_hopefuly(first_letter)?;
        let second: Outcome = outcome_from_str_hopefuly(second_letter)?;
        rounds.push(RoundChallenge2 { first, second })
    }
    Ok(rounds)
}

pub(crate) fn solve_day_2_challenge_2(input_path: &Path) -> anyhow::Result<Score> {
    let rounds = parse_input_file_challenge_2(input_path)?;
    let rounds_from_strategy: Vec<RoundChallenge1> = rounds
        .iter()
        .map(|r| RoundChallenge1 {
            first: r.first.clone(),
            second: shape_for_outcome(&r.first, &r.second),
        })
        .collect();
    Ok(score_rounds(rounds_from_strategy.as_slice()))
}

#[cfg(test)]
mod tests {
    use crate::solve_02::{solve_day_2_challenge_1, solve_day_2_challenge_2};
    use std::path::Path;

    #[test]
    fn test_solve_day_2_challenge_1() {
        assert_eq!(
            solve_day_2_challenge_1(Path::new("data/02.txt")).unwrap(),
            12586
        );
    }

    #[test]
    fn test_solve_day_2_challenge_2() {
        assert_eq!(
            solve_day_2_challenge_2(Path::new("data/02.txt")).unwrap(),
            13193
        );
    }
}
