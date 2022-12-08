use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::bail;
use itertools::Itertools;
use log::debug;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, not_line_ending};
use nom::character::streaming::multispace1;
use nom::IResult;
use nom::sequence::{tuple};
use crate::parser_utils::positive_number;
use crate::solve_07::TerminalLine::List;

#[derive(Debug,PartialEq)]
pub(crate) enum TerminalLine {
    ChangeDirectory(String),
    List,
    Directory(String),
    File(u64, String)
}

fn parse_change_directory(input: &str) -> IResult<&str, TerminalLine> {
    let (i, (_, _, path)) = tuple((
            tag("cd"),
            multispace1,
            not_line_ending,
    ))(input)?;
    Ok(
        (i, TerminalLine::ChangeDirectory(String::from(path)))
    )
}

fn parse_list(input: &str) -> IResult<&str, TerminalLine> {
    let (i, _) = tag("ls")(input)?;
    Ok((i, TerminalLine::List))
}

fn parse_command(input: &str) -> IResult<&str, TerminalLine> {
    let (i, (_, _, line)) = tuple((
        char('$'),
        multispace1,
        alt((
            parse_change_directory,
            parse_list,
        ))
    ))(input)?;
    Ok((i, line))

}

fn parse_directory(input: &str) -> IResult<&str, TerminalLine> {
    let (i, (_, _, directory_name)) = tuple((
        tag("dir"),
        multispace1,
        not_line_ending,
    ))(input)?;
    Ok((i, TerminalLine::Directory(String::from(directory_name))))
}

fn parse_file(input: &str) -> IResult<&str, TerminalLine> {
    let (i, (size, _, name)) = tuple((
        positive_number,
        multispace1,
        not_line_ending,
    ))(input)?;
    Ok((i, TerminalLine::File(size as u64, String::from(name))))
}

fn parse_terminal_line(input: &str) -> IResult<&str, TerminalLine> {
    let (i, line) = alt(
        (
            parse_command,
            parse_directory,
            parse_file
            ))(input)?;
    Ok((i, line))
}

pub(crate) fn parse_input(input_path: &Path) -> anyhow::Result<Vec<TerminalLine>> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let mut terminal_lines: Vec<TerminalLine> = vec![];
    for line in buffer_reader.lines() {
        let line = line?;
        match parse_terminal_line(line.as_str()) {
            Ok((_, terminal_line)) => {terminal_lines.push(terminal_line)}
            Err(_) => {bail!("failed to parse {:?}", line)}
        }
    }
    Ok(terminal_lines)
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Descriptor {
    File(u64, String),
    Folder(String),
}


fn size_folder(path: &String, flat_structure: &HashMap<String, HashSet<Descriptor>>) -> u64 {
    if let Some(content) = flat_structure.get(path) {
        content.iter().map(|descriptor| {
            match descriptor {
                Descriptor::File(size, _) => *size,
                Descriptor::Folder(name) => size_folder(&format!("{}/{}", path, name), flat_structure),
            }
        }).sum()
    } else {
        0
    }
}

fn build_flat_file_structure(terminal_lines: &[TerminalLine]) -> HashMap<String, HashSet<Descriptor>> {
    let mut current_path: Vec<String> = vec![];
    let mut flat_file_structure: HashMap<String, HashSet<Descriptor>> = HashMap::new();
    for terminal_line in terminal_lines.iter() {
        match terminal_line {
            TerminalLine::ChangeDirectory(new_directory) => {
                if new_directory.as_str() == ".." {
                    current_path.pop();
                } else {
                    current_path.push(String::from(new_directory));
                }
                debug!("current path: {:?}", current_path);
            }
            List => {}
            TerminalLine::Directory(name) => {
                let abs_path: String = current_path.iter().join("/");
                if let Some(folder) = flat_file_structure.get_mut(&abs_path) {
                    folder.insert(Descriptor::Folder(name.clone()));
                } else {
                    flat_file_structure.insert(abs_path, HashSet::from([Descriptor::Folder(name.clone())]));
                }
            }
            TerminalLine::File(size, name) => {
                let abs_path: String = current_path.iter().join("/");
                if let Some(folder) = flat_file_structure.get_mut(&abs_path) {
                    folder.insert(Descriptor::File(*size, name.clone()));
                } else {
                    flat_file_structure.insert(abs_path, HashSet::from([Descriptor::File(*size, name.clone())]));
                }
            }
        }
    }
    flat_file_structure
}

pub(crate) fn solve_day_7_challenge_1(input_path: &Path) -> anyhow::Result<u64> {
    let terminal_lines = parse_input(input_path)?;
    let flat_file_structure = build_flat_file_structure(terminal_lines.as_slice());

    Ok(flat_file_structure.iter().map(|(abs_path, _content)| {
        let size = size_folder(abs_path, &flat_file_structure);
        if size <= 100_000 {
            size
        } else {
            0
        }
    }).sum())
}

pub(crate) fn solve_day_7_challenge_2(input_path: &Path) -> anyhow::Result<Option<u64>> {
    let terminal_lines = parse_input(input_path)?;
    let flat_file_structure = build_flat_file_structure(terminal_lines.as_slice());

    let total_size = size_folder(&String::from("/"), &flat_file_structure);
    let unused_space: i64 = (70_000_000 - total_size) as i64;
    let space_to_free_up: i64 = 30_000_000 - unused_space;
    debug!("space to free up: {}", space_to_free_up);

    let victim = flat_file_structure.iter().filter_map(|(abs_path, _content)| {
        let size = size_folder(abs_path, &flat_file_structure);
        if size >= space_to_free_up as u64 {
            Some(size)
        } else {
            None
        }
    }).min();
    Ok(victim)
}

#[cfg(test)]
mod tests {
    use crate::solve_07::{TerminalLine, parse_change_directory, parse_command, parse_directory, parse_file, parse_list, parse_terminal_line};

    #[test]
    fn test_parse_change_directory() {
        assert_eq!(
            parse_change_directory("cd /"),
            Ok(("", TerminalLine::ChangeDirectory(String::from("/"))))
        )
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(
            parse_list("ls"),
            Ok(("", TerminalLine::List))
        )
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("$ cd foo"),
            Ok(("", TerminalLine::ChangeDirectory(String::from("foo"))))
        );
        assert_eq!(
            parse_command("$ ls"),
            Ok(("", TerminalLine::List))
        )
    }

    #[test]
    fn test_parse_directory() {
        assert_eq!(
            parse_directory("dir foo"),
            Ok(("", TerminalLine::Directory(String::from("foo"))))
        );
    }

    #[test]
    fn test_parse_file() {
        assert_eq!(
            parse_file("1234 foo.rs"),
            Ok(("", TerminalLine::File(1234, String::from("foo.rs"))))
        );
    }

    #[test]
    fn test_parse_terminal_line() {
        assert_eq!(
            parse_terminal_line("$ ls"),
            Ok(("", TerminalLine::List))
        );
        assert_eq!(
            parse_terminal_line("$ cd foo"),
            Ok(("", TerminalLine::ChangeDirectory(String::from("foo"))))
        );
        assert_eq!(
            parse_terminal_line("dir foo"),
            Ok(("", TerminalLine::Directory(String::from("foo"))))
        );
        assert_eq!(
            parse_terminal_line("1234 foo"),
            Ok(("", TerminalLine::File(1234, String::from("foo"))))
        )
    }
}