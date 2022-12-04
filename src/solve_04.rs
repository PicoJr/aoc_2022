use log::debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

type Score = usize;

#[derive(Debug)]
struct SectionRange {
    begin: usize,
    end: usize,
}

fn parse_range(s: &str) -> anyhow::Result<SectionRange> {
    let range_bounds: Vec<&str> = s.split('-').collect();
    let first_bound = range_bounds
        .first()
        .ok_or_else(|| anyhow::anyhow!("parsing error {}", s))?;
    let first_bound = usize::from_str(first_bound)?;
    let second_bound = range_bounds
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("parsing error {}", s))?;
    let second_bound = usize::from_str(second_bound)?;
    Ok(SectionRange {
        begin: first_bound,
        end: second_bound,
    })
}

impl SectionRange {
    pub fn inside(&self, other: &Self) -> bool {
        other.begin <= self.begin && self.end <= other.end
    }

    pub fn overlap(&self, other: &Self) -> bool {
        other.begin <= self.begin && self.begin <= other.end
            || other.begin <= self.end && self.end <= other.end
            || other.inside(self)
    }
}

type ElfPair = (SectionRange, SectionRange);

fn parse_input_file(input_path: &Path) -> anyhow::Result<Vec<ElfPair>> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let mut assignments: Vec<ElfPair> = vec![];
    for line in buffer_reader.lines() {
        let line = line?;
        let ranges: Vec<&str> = line.split(',').collect();
        let first_range = ranges
            .first()
            .ok_or_else(|| anyhow::anyhow!("parsing error {}", line))?;
        let first_range = parse_range(first_range)?;
        let second_range = ranges
            .get(1)
            .ok_or_else(|| anyhow::anyhow!("parsing error {}", line))?;
        let second_range = parse_range(second_range)?;

        assignments.push((first_range, second_range));
    }
    Ok(assignments)
}

pub(crate) fn solve_day_4_challenge_1(input_path: &Path) -> anyhow::Result<Score> {
    let assignments = parse_input_file(input_path)?;
    let score = assignments
        .iter()
        .map(|(section_range_1, section_range_2)| {
            if section_range_1.inside(section_range_2) || section_range_2.inside(section_range_1) {
                debug!(
                    "{:?} in {:?} or {:?} in {:?}",
                    section_range_1, section_range_2, section_range_2, section_range_1
                );
                1
            } else {
                0
            }
        })
        .sum();
    Ok(score)
}

pub(crate) fn solve_day_4_challenge_2(input_path: &Path) -> anyhow::Result<Score> {
    let assignments = parse_input_file(input_path)?;
    let score = assignments
        .iter()
        .map(|(section_range_1, section_range_2)| {
            if section_range_1.overlap(section_range_2) {
                debug!("{:?} overlaps {:?}", section_range_1, section_range_2);
                1
            } else {
                debug!(
                    "{:?} does not overlap {:?}",
                    section_range_1, section_range_2
                );
                0
            }
        })
        .sum();
    Ok(score)
}
