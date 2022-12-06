use itertools::Itertools;
use log::debug;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

type Score = usize;

fn parse_input(input_path: &Path) -> anyhow::Result<String> {
    let input_file = File::open(input_path)?;
    let mut buffer_reader = BufReader::new(input_file);
    let mut buffer = String::new();
    buffer_reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub(crate) fn solve_day_6_challenge_1(input_path: &Path) -> anyhow::Result<Option<Score>> {
    let stream = parse_input(input_path)?;
    for (count, (c0, c1, c2, c3)) in stream.chars().into_iter().tuple_windows().enumerate() {
        if c0 != c1 && c0 != c2 && c0 != c3 && c1 != c2 && c1 != c3 && c2 != c3 {
            debug!(
                "found {}{}{}{} after reading {} chars",
                c0, c1, c2, c3, count
            );
            return Ok(Some(count + 4));
        }
    }
    Ok(None)
}

pub(crate) fn solve_day_6_challenge_2(input_path: &Path) -> anyhow::Result<Option<Score>> {
    let stream = parse_input(input_path)?;
    let chars: Vec<char> = stream.chars().into_iter().collect();
    for pos in 0..chars.len() {
        let window = &chars.as_slice()[pos..pos + 14];
        let set: HashSet<&char> = HashSet::from_iter(window.iter());
        if set.len() == 14 {
            debug!("found {:?} after reading {} chars", window, pos);
            return Ok(Some(pos + 14));
        }
    }
    Ok(None)
}
