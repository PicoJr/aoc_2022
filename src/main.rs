extern crate core;

mod cli;
mod parser_utils;
mod solve_01;
mod solve_02;
mod solve_03;
mod solve_04;
mod solve_05;
mod solve_06;
mod solve_07;
mod solve_08;
mod solve_09;
mod solve_10;
mod solve_11;

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
        Args {
            day: 7,
            challenge: 1,
            data_path,
        } => {
            let score = solve_07::solve_day_7_challenge_1(&data_path.as_path().join("07.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 7,
            challenge: 2,
            data_path,
        } => {
            let score = solve_07::solve_day_7_challenge_2(&data_path.as_path().join("07.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 8,
            challenge: 1,
            data_path,
        } => {
            let score = solve_08::solve_day_8_challenge_1(&data_path.as_path().join("08.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 8,
            challenge: 2,
            data_path,
        } => {
            let score = solve_08::solve_day_8_challenge_2(&data_path.as_path().join("08.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 9,
            challenge: 1,
            data_path,
        } => {
            let score = solve_09::solve_day_9_challenge_1(&data_path.as_path().join("09.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 9,
            challenge: 2,
            data_path,
        } => {
            let score = solve_09::solve_day_9_challenge_2(&data_path.as_path().join("09.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 10,
            challenge: 1,
            data_path,
        } => {
            let score = solve_10::solve_day_10_challenge_1(&data_path.as_path().join("10.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 10,
            challenge: 2,
            data_path,
        } => {
            let score = solve_10::solve_day_10_challenge_2(&data_path.as_path().join("10.txt"))?;
            println!("{:?}", score);
        }
        Args {
            day: 11,
            challenge: 1,
            data_path: _,
        } => {
            let score = solve_11::solve_day_11_challenge_1()?;
            println!("{:?}", score);
        }
        Args {
            day: 11,
            challenge: 2,
            data_path: _,
        } => {
            let score = solve_11::solve_day_11_challenge_2()?;
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
