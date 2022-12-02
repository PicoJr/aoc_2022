mod cli;
mod solve_01;

use crate::cli::Args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args: Args = cli::Args::parse();
    match args {
        Args {
            day: 1,
            challenge: 1,
            data_path,
        } => {
            let calories = solve_01::solve_calorie_counting(&data_path.as_path().join("01.txt"), 1);
            if let Ok(calories) = calories {
                println!("{}", calories);
            }
        }
        Args {
            day: 1,
            challenge: 2,
            data_path,
        } => {
            let calories = solve_01::solve_calorie_counting(&data_path.as_path().join("01.txt"), 3);
            if let Ok(calories) = calories {
                println!("{}", calories);
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
