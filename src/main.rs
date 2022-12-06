extern crate core;

mod cli;
mod solve_01;
mod solve_02;
mod solve_03;
mod solve_04;
mod solve_05;
mod solve_06;

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
            let calories =
                solve_01::solve_day_1_challenges(&data_path.as_path().join("01.txt"), 1)?;
            println!("{:?}", calories);
        }
        Args {
            day: 1,
            challenge: 2,
            data_path,
        } => {
            let calories =
                solve_01::solve_day_1_challenges(&data_path.as_path().join("01.txt"), 3)?;
            println!("{:?}", calories);
        }
        Args {
            day: 2,
            challenge: 1,
            data_path,
        } => {
            let score = solve_02::solve_day_2_challenge_1(&data_path.as_path().join("02.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 2,
            challenge: 2,
            data_path,
        } => {
            let score = solve_02::solve_day_2_challenge_2(&data_path.as_path().join("02.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 3,
            challenge: 1,
            data_path,
        } => {
            let score = solve_03::solve_day_3_challenge_1(&data_path.as_path().join("03.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 3,
            challenge: 2,
            data_path,
        } => {
            let score = solve_03::solve_day_3_challenge_2(&data_path.as_path().join("03.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 4,
            challenge: 1,
            data_path,
        } => {
            let score = solve_04::solve_day_4_challenge_1(&data_path.as_path().join("04.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 4,
            challenge: 2,
            data_path,
        } => {
            let score = solve_04::solve_day_4_challenge_2(&data_path.as_path().join("04.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 5,
            challenge: 1,
            data_path,
        } => {
            let score =
                solve_05::solve_day_5_challenge_1(&data_path.as_path().join("05.txt"), false)?;
            println!("{:?}", score);
        }
        Args {
            day: 5,
            challenge: 2,
            data_path,
        } => {
            let score =
                solve_05::solve_day_5_challenge_1(&data_path.as_path().join("05.txt"), true)?;
            println!("{:?}", score);
        }
        Args {
            day: 6,
            challenge: 1,
            data_path,
        } => {
            let score = solve_06::solve_day_6_challenge_1(&data_path.as_path().join("06.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 6,
            challenge: 2,
            data_path,
        } => {
            let score = solve_06::solve_day_6_challenge_2(&data_path.as_path().join("06.txt"))?;
            println!("{:?}", score);
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
