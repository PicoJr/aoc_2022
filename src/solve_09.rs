use crate::parser_utils::{positive_number, single_space};
use crate::solve_09::Direction::{Down, Left, Right, Up};
use anyhow::bail;
use itertools::repeat_n;
use log::debug;
use nalgebra::{Point2, Vector2};
use nom::branch::alt;
use nom::character::complete::char;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Score = u64;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_direction_and_steps(input: &str) -> IResult<&str, (Direction, u64)> {
    let (i, (direction, _, steps)) = tuple((
        alt((char('U'), char('D'), char('L'), char('R'))),
        single_space,
        positive_number,
    ))(input)?;
    match direction {
        'U' => Ok((i, (Up, steps as u64))),
        'D' => Ok((i, (Down, steps as u64))),
        'L' => Ok((i, (Left, steps as u64))),
        'R' => Ok((i, (Right, steps as u64))),
        _ => unreachable!(), // because we match against U D L R using nom
    }
}

pub fn parse_input(input_path: &Path) -> anyhow::Result<Vec<(Direction, u64)>> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let mut directions_and_steps = vec![];
    for line in buffer_reader.lines() {
        let line = line?;
        if let Ok((_i, (direction, steps))) = parse_direction_and_steps(line.as_str()) {
            directions_and_steps.push((direction, steps));
        } else {
            bail!("failed to parse {}", line);
        }
    }
    Ok(directions_and_steps)
}

fn planckerize(direction: &Direction, steps: u64) -> impl Iterator<Item = Vector2<i64>> {
    let unit_vector = match direction {
        Up => Vector2::new(0, -1),
        Down => Vector2::new(0, 1),
        Left => Vector2::new(-1, 0),
        Right => Vector2::new(1, 0),
    };
    repeat_n(unit_vector, steps as usize)
}

fn adjacent_or_overlapping(p1: &Point2<i64>, p2: &Point2<i64>) -> bool {
    (i64::abs(p1.x - p2.x) <= 1) && (i64::abs(p1.y - p2.y) <= 1)
}

fn vector_to_head(tail_position: &Point2<i64>, head_position: &Point2<i64>) -> Vector2<i64> {
    if adjacent_or_overlapping(tail_position, head_position) {
        Vector2::new(0, 0) // no need to move
    } else {
        let to_head = head_position - tail_position;
        Vector2::new(to_head.x.clamp(-1, 1), to_head.y.clamp(-1, 1))
    }
}

pub fn solve_day_9_challenge_1(input_path: &Path) -> anyhow::Result<Score> {
    let directions_and_steps = parse_input(input_path)?;
    let mut head_position: Point2<i64> = Point2::from([0, 0]);
    let mut tail_position: Point2<i64> = Point2::from([0, 0]);

    let mut tail_positions: HashSet<Point2<i64>> = HashSet::new();
    tail_positions.insert(tail_position);

    for (direction, steps) in directions_and_steps {
        debug!("direction {:?}, steps: {}", direction, steps);
        for dposition in planckerize(&direction, steps) {
            debug!(
                "head position: {} -> {}",
                head_position,
                head_position + dposition
            );
            head_position += dposition;
            let dtail = vector_to_head(&tail_position, &head_position);
            debug!(
                "tail position: {} -> {}",
                tail_position,
                tail_position + dtail
            );
            tail_position += dtail;
            assert!(adjacent_or_overlapping(&tail_position, &head_position));
            tail_positions.insert(tail_position);
        }
    }
    Ok(tail_positions.len() as Score)
}

pub fn solve_day_9_challenge_2(input_path: &Path) -> anyhow::Result<Score> {
    let directions_and_steps = parse_input(input_path)?;
    let mut head_position: Point2<i64> = Point2::from([0, 0]);
    let mut tail: Vec<Point2<i64>> = repeat_n(Point2::from([0, 0]), 9).collect();

    let mut tail_positions: HashSet<Point2<i64>> = HashSet::new();
    tail_positions.insert(Point2::from([0, 0]));

    for (direction, steps) in directions_and_steps {
        debug!("direction {:?}, steps: {}", direction, steps);
        for dposition in planckerize(&direction, steps) {
            debug!(
                "head position: {} -> {}",
                head_position,
                head_position + dposition
            );
            head_position += dposition;

            let mut target: Point2<i64> = head_position;
            for (i, tail_part) in tail.iter_mut().enumerate().take(9) {
                let dtail = vector_to_head(tail_part, &target);
                debug!("tail position: {} -> {}", tail_part, *tail_part + dtail);
                *tail_part += dtail;
                assert!(adjacent_or_overlapping(tail_part, &target));
                target = *tail_part;

                if i == 8 {
                    tail_positions.insert(*tail_part);
                }
            }
        }
    }
    Ok(tail_positions.len() as Score)
}
