use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

type Calories = u64;
pub(crate) fn solve_day_1_challenges(input_path: &Path, n: usize) -> anyhow::Result<Calories> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let mut elves_calories: Vec<Calories> = vec![];
    let mut elve_calories_sum: Calories = 0;
    for line in buffer_reader.lines() {
        let line = line?;
        if line.is_empty() {
            elves_calories.push(elve_calories_sum);
            elve_calories_sum = 0; // reset calories
        } else {
            elve_calories_sum += u64::from_str(line.as_str())?;
        }
    }
    elves_calories.sort_unstable_by(|a, b| b.cmp(a)); // sort decreasing
    Ok(elves_calories.iter().take(n).sum())
}

#[cfg(test)]
mod tests {
    use crate::solve_01::solve_day_1_challenges;
    use std::path::Path;

    #[test]
    fn test_solve_calorie_counting() {
        assert_eq!(
            solve_day_1_challenges(Path::new("data/01.txt"), 1).unwrap(),
            69310
        );
        assert_eq!(
            solve_day_1_challenges(Path::new("data/01.txt"), 3).unwrap(),
            206104
        )
    }
}
