use crate::parser_utils::positive_number;
use itertools::Itertools;
use log::debug;
use nalgebra::{min, DMatrix};
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Score = u64;

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (i, (x, y)) = separated_pair(positive_number, char(','), positive_number)(input)?;
    Ok((i, Point { x, y }))
}

fn parse_segments(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), parse_point)(input)
}

#[cfg(test)]
mod tests {
    use crate::solve_14::{parse_segments, Point};

    #[test]
    fn test_parse_segments() {
        assert_eq!(
            parse_segments("42,43 -> 44,45"),
            Ok(("", vec![Point { x: 42, y: 43 }, Point { x: 44, y: 45 }]))
        );
    }
}

#[derive(Debug)]
pub struct RockStructure {
    rocks: DMatrix<u32>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

pub(crate) fn parse_input(input_path: &Path) -> anyhow::Result<RockStructure> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let mut rock_segments: Vec<Vec<Point>> = vec![];
    for line in buffer_reader.lines() {
        let line = line?;
        if let Ok((_i, segments)) = parse_segments(line.as_str()) {
            rock_segments.push(segments);
        }
    }
    let min_x = rock_segments
        .iter()
        .flatten()
        .map(|p| p.x)
        .min()
        .ok_or(anyhow::anyhow!("could not compute min for x"))?;
    let max_x = rock_segments
        .iter()
        .flatten()
        .map(|p| p.x)
        .max()
        .ok_or(anyhow::anyhow!("could not compute max for y"))?;
    let min_y = rock_segments
        .iter()
        .flatten()
        .map(|p| p.y)
        .min()
        .ok_or(anyhow::anyhow!("could not compute min for x"))?;
    let max_y = rock_segments
        .iter()
        .flatten()
        .map(|p| p.y)
        .max()
        .ok_or(anyhow::anyhow!("could not compute max for y"))?;
    debug!("x:{} {} y:{} {}", min_x, max_x, min_y, max_y);
    let rows = max_y + 1; // 0 -> max_y
    let cols = max_x - min_x + 1; // 0 -> max_x - min_x
    let mut rocks = DMatrix::zeros(rows, cols);
    for segments in rock_segments {
        for (p1, p2) in segments.iter().tuple_windows() {
            if p1.x == p2.x {
                for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                    rocks[(y, p1.x - min_x)] = 1;
                }
            }
            if p1.y == p2.y {
                for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
                    rocks[(p1.y, x - min_x)] = 1;
                }
            }
        }
    }

    for row in rocks.row_iter() {
        debug!(
            "{:?}",
            row.iter().map(|x| if *x == 1 { '#' } else { '.' }).join("")
        );
    }

    Ok(RockStructure {
        rocks,
        min_x,
        max_x,
        min_y,
        max_y,
    })
}

pub(crate) fn solve_day_14_challenge_1(input_path: &Path) -> anyhow::Result<Score> {
    let mut rock_structure = parse_input(input_path)?;

    let mut sand: Point = Point { x: 500, y: 0 };
    let mut outside = false;
    let mut score: Score = 0;
    while !outside {
        if sand.y + 1 > rock_structure.max_y {
            outside = true;
            break;
        }
        if rock_structure.rocks[(sand.y + 1, sand.x - rock_structure.min_x)] == 0 {
            // empty, sand can move there
            sand.y += 1;
            continue;
        }
        // not empty below
        // check left
        if sand.x - rock_structure.min_x == 0 {
            outside = true;
            break;
        }
        // left not outside
        // check diagonally left
        if rock_structure.rocks[(sand.y + 1, sand.x - rock_structure.min_x - 1)] == 0 {
            // empty, sand can move there
            sand.y += 1;
            sand.x -= 1;
            continue;
        }
        // not empty diagonally left
        // check right
        if sand.x == rock_structure.max_x {
            outside = true;
            break;
        }
        // right not outside
        // check diagonally right
        if rock_structure.rocks[(sand.y + 1, sand.x - rock_structure.min_x + 1)] == 0 {
            // empty, sand can move there
            sand.y += 1;
            sand.x += 1;
            continue;
        }

        // could not move bottom, diagonally left or diagonally right -> rest
        rock_structure.rocks[(sand.y, sand.x - rock_structure.min_x)] = 2;
        // debug!("resting here: {:?}", sand);
        score += 1;
        sand = Point { x: 500, y: 0 };
    }

    for row in rock_structure.rocks.row_iter() {
        debug!(
            "{:?}",
            row.iter()
                .map(|x| match x {
                    0 => '.',
                    1 => '#',
                    2 => 'o',
                    _ => '?',
                })
                .join("")
        );
    }
    Ok(score)
}

pub(crate) fn solve_day_14_challenge_2(input_path: &Path) -> anyhow::Result<Score> {
    let rock_structure = parse_input(input_path)?;

    let rows = rock_structure.rocks.nrows();
    let cols = rock_structure.rocks.ncols();

    let margin = rows + 2;
    let mut rocks_challenge_2 = DMatrix::zeros(rows + 2, margin + cols + margin);
    for r in 0..rock_structure.rocks.nrows() {
        for c in 0..rock_structure.rocks.ncols() {
            rocks_challenge_2[(r, c + margin)] = rock_structure.rocks[(r, c)];
        }
    }

    let last_row = rocks_challenge_2.nrows() - 1;
    for c in 0..rocks_challenge_2.ncols() {
        rocks_challenge_2[(last_row, c)] = 1;
    }

    let mut sand: Point = Point { x: 500, y: 0 };
    let mut score: Score = 0;
    loop {
        // debug!("sand: {:?}, margin {:?}", sand, margin);
        if rocks_challenge_2[(sand.y + 1, sand.x + margin - rock_structure.min_x)] == 0 {
            // empty, sand can move there
            sand.y += 1;
            continue;
        }
        // not empty below
        // check left
        if sand.x == rock_structure.min_x - margin {
            debug!("should not happen thanks to margins");
            break;
        }
        // left not outside
        // check diagonally left
        if rocks_challenge_2[(sand.y + 1, sand.x + margin - rock_structure.min_x - 1)] == 0 {
            // empty, sand can move there
            sand.y += 1;
            sand.x -= 1;
            continue;
        }
        // not empty diagonally left
        // check right
        if sand.x == rock_structure.max_x + margin {
            debug!("should not happen thanks to margins");
            break;
        }
        // right not outside
        // check diagonally right
        if rocks_challenge_2[(sand.y + 1, sand.x + margin - rock_structure.min_x + 1)] == 0 {
            // empty, sand can move there
            sand.y += 1;
            sand.x += 1;
            continue;
        }
        // check if we reached the top row
        if sand.y == 0 {
            rocks_challenge_2[(sand.y, sand.x + margin - rock_structure.min_x)] = 2;
            score += 1;
            break;
        }

        // could not move bottom, diagonally left or diagonally right -> rest
        rocks_challenge_2[(sand.y, sand.x + margin - rock_structure.min_x)] = 2;
        // debug!("resting here: {:?}", sand);
        score += 1;
        sand = Point { x: 500, y: 0 };
    }

    for row in rocks_challenge_2.row_iter() {
        debug!(
            "{:?}",
            row.iter()
                .map(|x| match x {
                    0 => '.',
                    1 => '#',
                    2 => 'o',
                    _ => '?',
                })
                .join("")
        );
    }
    Ok(score)
}
