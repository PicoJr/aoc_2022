use crate::parser_utils::{positive_number, single_space};
use crate::solve_10::Instruction::{Addx, Noop};
use log::debug;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Score = i64;

#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    Addx(i64),
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("noop")(input)?;
    Ok((i, Noop))
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (i, (_addx, _, sign, x)) = tuple((
        tag("addx"),
        single_space,
        opt(nom::character::complete::char('-')),
        positive_number,
    ))(input)?;
    let v = if sign.is_some() {
        -(x as i64)
    } else {
        x as i64
    };
    Ok((i, Addx(v)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_noop, parse_addx))(input)
}

pub(crate) fn parse_input(input_path: &Path) -> anyhow::Result<Vec<Instruction>> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let mut instructions = vec![];
    for line in buffer_reader.lines() {
        let line = line?;
        if let Ok((_, instruction)) = parse_instruction(line.as_str()) {
            instructions.push(instruction)
        }
    }
    Ok(instructions)
}

// cycle when instruction effect will take place, instruction
type DelayedInstruction = (u64, Instruction);

pub(crate) fn solve_day_10_challenge_1(input_path: &Path) -> anyhow::Result<Score> {
    let instructions = parse_input(input_path)?;
    let mut delayed: VecDeque<DelayedInstruction> = VecDeque::new();
    let mut cycle: u64 = 0;
    for instruction in instructions {
        match instruction {
            Noop => {
                cycle += 1;
            }
            Addx(x) => {
                delayed.push_back((cycle + 2, Addx(x)));
                cycle += 2;
            }
        }
    }

    let mut x_register: i64 = 1;
    let mut score: i64 = 0;
    for signal_cycle in [20, 60, 100, 140, 180, 220] {
        while !delayed.is_empty() && (delayed.front().expect("not empty")).0 < signal_cycle {
            let (_, instruction) = delayed.pop_front().expect("not empty");

            if let Addx(x) = instruction {
                x_register += x
            }
        }
        debug!("x register for cycle {} is {}", signal_cycle, x_register);
        score += signal_cycle as i64 * x_register;
    }

    Ok(score)
}

pub(crate) fn solve_day_10_challenge_2(input_path: &Path) -> anyhow::Result<Score> {
    let instructions = parse_input(input_path)?;
    let mut delayed: VecDeque<DelayedInstruction> = VecDeque::new();
    let mut cycle: u64 = 1;

    for instruction in instructions {
        match instruction {
            Noop => {
                cycle += 1;
            }
            Addx(x) => {
                delayed.push_back((cycle + 2, Addx(x)));
                cycle += 2;
            }
        }
    }

    let mut x_register: i64 = 1;
    let mut pixels: Vec<char> = vec![];
    for signal_cycle in 1..=240 {
        let crt_pixel_position = (signal_cycle - 1) % 40;
        while !delayed.is_empty() && (delayed.front().expect("not empty")).0 <= signal_cycle {
            let (_, instruction) = delayed.pop_front().expect("not empty");

            if let Addx(x) = instruction {
                x_register += x
            }
        }

        let p = if (x_register == (crt_pixel_position as i64 - 1))
            || (x_register == (crt_pixel_position as i64))
            || (x_register == (crt_pixel_position as i64 + 1))
        {
            '#'
        } else {
            '.'
        };
        // debug!("cycle: {}, x register: {}, pixel: {}", signal_cycle, x_register, p);
        pixels.push(p)
    }

    // debug!("pixels: {:?}", pixels);
    for chunk in pixels.chunks(40) {
        println!("{}", String::from_iter(chunk.iter()));
    }

    Ok(0)
}
