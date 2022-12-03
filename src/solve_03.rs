use itertools::Itertools;
use log::debug;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Score = u64;

fn parse_input_file(input_path: &Path) -> anyhow::Result<Vec<String>> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let mut contents = vec![];
    for line in buffer_reader.lines() {
        let line = line?;
        contents.push(line);
    }
    Ok(contents)
}

fn split_rucksack(content: &String) -> anyhow::Result<(String, String)> {
    if (content.len() % 2) != 0 {
        anyhow::bail!("rucksack contain uneven compartments")
    } else {
        let (first, second) = content.split_at(content.len() / 2);
        Ok((String::from(first), String::from(second)))
    }
}

fn shared_item_types(first: &str, second: &str) -> Vec<char> {
    let first_set: HashSet<char> = HashSet::from_iter(first.chars());
    let second_set: HashSet<char> = HashSet::from_iter(second.chars());
    first_set.intersection(&second_set).cloned().collect()
}

fn priority(c: &char) -> Score {
    let code = *c as u32;
    if ('a' as u32) <= code && code <= ('z' as u32) {
        (code - ('a' as u32) + 1) as u64
    } else if ('A' as u32) <= code && code <= ('Z' as u32) {
        (code - ('A' as u32) + 27) as u64
    } else {
        0
    }
}

pub(crate) fn solve_day_3_challenge_1(input_path: &Path) -> anyhow::Result<Score> {
    let contents = parse_input_file(input_path)?;
    let total: anyhow::Result<Vec<Score>> = contents
        .iter()
        .map(|content| {
            let (first_compartment, second_compartment) = split_rucksack(content)?;
            let shared_items = shared_item_types(&first_compartment, &second_compartment);
            debug!("{:?}", shared_items);
            Ok(shared_items
                .iter()
                .map(|c| {
                    debug!("priority of '{}' == {}", c, priority(c));
                    priority(c)
                })
                .sum::<Score>())
        })
        .collect();
    Ok(total?.iter().sum())
}

pub(crate) fn solve_day_3_challenge_2(input_path: &Path) -> anyhow::Result<Score> {
    let contents = parse_input_file(input_path)?;
    let mut total: Score = 0;
    for (content_0, content_1, content_2) in contents.into_iter().tuples() {
        let intersection_0_1 = shared_item_types(&content_0, &content_1);
        let intersection_0_2 = shared_item_types(&content_0, &content_2);
        let first_set: HashSet<char> = HashSet::from_iter(intersection_0_1.into_iter());
        let second_set: HashSet<char> = HashSet::from_iter(intersection_0_2.into_iter());
        let intersection_0_1_2 = first_set.intersection(&second_set);
        debug!("{:?}", intersection_0_1_2);
        let s: Score = intersection_0_1_2
            .into_iter()
            .map(|c| {
                debug!("priority of '{}' == {}", c, priority(c));
                priority(c)
            })
            .sum();
        total += s;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use crate::solve_03::{solve_day_3_challenge_1, solve_day_3_challenge_2};
    use std::path::Path;

    #[test]
    fn test_solve_day_3_challenge_1() {
        assert_eq!(
            solve_day_3_challenge_1(Path::new("data/03.txt")).unwrap(),
            8401
        );
    }

    #[test]
    fn test_solve_day_3_challenge_2() {
        assert_eq!(
            solve_day_3_challenge_2(Path::new("data/03.txt")).unwrap(),
            2641
        );
    }
}
