use anyhow::{anyhow, bail};
use log::debug;
use nom::bytes::complete::{tag, take_until, take_while_m_n};
use nom::character::complete::{anychar, digit1, line_ending};
use nom::combinator::map_res;
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn number(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn positive_number(input: &str) -> IResult<&str, usize> {
    map_res(number, |out| out.parse::<usize>())(input)
}

fn single_space(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, |c: char| c == ' ')(input)
}

fn parse_move_line(input: &str) -> IResult<&str, Move> {
    let (i, (_move, _, count, _, _from, _, from, _, _to, _, to)) = tuple((
        tag("move"),
        single_space,
        positive_number,
        single_space,
        tag("from"),
        single_space,
        positive_number,
        single_space,
        tag("to"),
        single_space,
        positive_number,
    ))(input)?;
    Ok((i, Move { count, from, to }))
}

fn parse_move_line_eol(input: &str) -> IResult<&str, Move> {
    terminated(parse_move_line, line_ending)(input)
}

fn parse_crate_char(input: &str) -> IResult<&str, char> {
    delimited(tag("["), anychar, tag("]"))(input)
}

fn maybe_crate(input: &str) -> IResult<&str, Option<char>> {
    match parse_crate_char(input) {
        Ok((i, c)) => Ok((i, Some(c))),
        Err(_) => match tag("   ")(input) {
            Ok((i, _c)) => Ok((i, None)),
            Err(e) => Err(e),
        },
    }
}

fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list0(tag(" "), maybe_crate)(input)
}

fn parse_crate_line_eol(input: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(parse_crate_line, line_ending)(input)
}

#[derive(Debug, Eq, PartialEq)]
struct Input {
    crates: Vec<Vec<Option<char>>>, // top -> bottom, left -> right
    moves: Vec<Move>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (i, crates) = many1(parse_crate_line_eol)(input)?;
    let (i, _v) = take_until("move")(i)?;
    let (i, moves) = many0(parse_move_line_eol)(i)?;
    Ok((i, Input { crates, moves }))
}

pub(crate) fn solve_day_5_challenge_1(
    input_path: &Path,
    rev_stacks: bool,
) -> anyhow::Result<String> {
    let input_file = File::open(input_path)?;
    let mut buffer_reader = BufReader::new(input_file);
    let mut buffer = String::new();
    let _read = buffer_reader.read_to_string(&mut buffer)?;
    match parse_input(buffer.as_str()) {
        Ok((_leftovers, input)) => {
            let stacks_n = input.crates.first().map(|e| e.len());
            if let Some(stacks_n) = stacks_n {
                let mut stacks: Vec<Vec<char>> = vec![];
                for _stack_id in 0..stacks_n {
                    stacks.push(vec![]);
                }
                for crate_line_bottom_up in input.crates.iter().rev() {
                    for (crate_stack, crate_maybe) in crate_line_bottom_up.iter().enumerate() {
                        if let Some(crate_name) = crate_maybe {
                            stacks
                                .get_mut(crate_stack)
                                .ok_or_else(|| anyhow!("stack id"))?
                                .push(*crate_name);
                        }
                    }
                }
                for m in input.moves.iter() {
                    debug!("move {} from {} to {}", m.count, m.from, m.to);
                    let from_indexed_from_0 = m.from - 1;
                    let to_indexed_from_0 = m.to - 1;
                    let from_stack = stacks
                        .get(from_indexed_from_0)
                        .ok_or_else(|| anyhow!("stack from"))?;
                    // I am not proud of the following line ^^'
                    let mut from =
                        VecDeque::from_iter(from_stack.iter().rev().take(m.count).rev().cloned());
                    debug!("{:?}", from);
                    for _c in 0..m.count {
                        if let Some(stack_to) = stacks.get_mut(to_indexed_from_0) {
                            let v = if rev_stacks {
                                from.pop_front().ok_or_else(|| anyhow!("stack pop"))?
                            } else {
                                from.pop_back().ok_or_else(|| anyhow!("stack pop"))?
                            };
                            debug!("{} moved from {} to {}", v, m.from, m.to);
                            stack_to.push(v);
                        }
                        if let Some(stack_from) = stacks.get_mut(from_indexed_from_0) {
                            stack_from.pop();
                        }
                    }
                }
                let top_of_each_stack: String =
                    stacks.iter().flat_map(|s| s.last().copied()).collect();
                return Ok(top_of_each_stack);
            }
        }
        Err(_) => {
            eprintln!("failed to parse input {:?}", &input_path);
        }
    }
    bail!("merry christmas");
}

#[cfg(test)]
mod tests {
    use crate::solve_05::{
        maybe_crate, parse_crate_char, parse_crate_line, parse_input, parse_move_line, Input, Move,
    };

    #[test]
    fn test_parse_move() {
        assert_eq!(
            parse_move_line("move 3 from 8 to 2"),
            Ok((
                "",
                Move {
                    count: 3,
                    from: 8,
                    to: 2,
                }
            ))
        );
    }

    #[test]
    fn test_parse_crate_char() {
        assert_eq!(parse_crate_char("[R]"), Ok(("", 'R')))
    }

    #[test]
    fn test_maybe_crate() {
        assert_eq!(maybe_crate("[A]"), Ok(("", Some('A'))));
        assert_eq!(maybe_crate("   "), Ok(("", None)))
    }

    #[test]
    fn test_parse_crate_line() {
        assert_eq!(
            parse_crate_line("[A] [B] [C]"),
            Ok(("", vec![Some('A'), Some('B'), Some('C')]))
        );
        assert_eq!(
            parse_crate_line("[A]     [C]"),
            Ok(("", vec![Some('A'), None, Some('C')]))
        );
        assert_eq!(
            parse_crate_line("        [C]"),
            Ok(("", vec![None, None, Some('C')]))
        );
        assert_eq!(
            parse_crate_line("        [C]    "),
            Ok(("", vec![None, None, Some('C'), None]))
        )
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("[A]\n 1 \n\nmove 1 from 2 to 3\n"),
            Ok((
                "",
                Input {
                    crates: vec![vec![Some('A')]],
                    moves: vec![Move {
                        count: 1,
                        from: 2,
                        to: 3,
                    }]
                }
            ))
        )
    }
}
