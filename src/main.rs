mod cli;
mod solve_01;
mod solve_02;

use crate::cli::Args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args: Args = cli::Args::parse();
    match args {
        Args {
            day: 1,
            challenge: 1,
            data_path,
        } => {
            let calories = solve_01::solve_day_1_challenges(&data_path.as_path().join("01.txt"), 1);
            if let Ok(calories) = calories {
                println!("{}", calories);
            }
        }
        Args {
            day: 1,
            challenge: 2,
            data_path,
        } => {
            let calories = solve_01::solve_day_1_challenges(&data_path.as_path().join("01.txt"), 3);
            if let Ok(calories) = calories {
                println!("{}", calories);
            }
        }
        Args {
            day: 2,
            challenge: 1,
            data_path,
        } => {
            let score = solve_02::solve_day_2_challenge_1(&data_path.as_path().join("02.txt"));
            if let Ok(score) = score {
                println!("{}", score);
            }
        }
        Args {
            day: 2,
            challenge: 2,
            data_path,
        } => {
            let score = solve_02::solve_day_2_challenge_2(&data_path.as_path().join("02.txt"));
            if let Ok(score) = score {
                println!("{}", score);
            }
        }
        Args { day, challenge, .. } => {
            eprintln!(
                "no solver available for day {}, challenge {}",
                day, challenge
            );
        }
    }
    Ok(())
}
