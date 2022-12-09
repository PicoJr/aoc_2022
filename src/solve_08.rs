use log::debug;
use nalgebra::{DMatrix, RowDVector};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Score = u64;
type Forest = DMatrix<i32>;

pub(crate) fn parse_input(input_path: &Path) -> anyhow::Result<Forest> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let rows: Vec<RowDVector<i32>> = buffer_reader
        .lines()
        .flat_map(|line| {
            if let Ok(line) = line {
                let canopees = line.chars().map(|c| c as i32 - '0' as i32).into_iter();
                Some(RowDVector::from_iterator(line.len(), canopees))
            } else {
                None
            }
        })
        .collect();
    let forest: Forest = DMatrix::from_rows(rows.as_slice());
    Ok(forest)
}

pub(crate) fn solve_day_8_challenge_1(input_path: &Path) -> anyhow::Result<Score> {
    let forest = parse_input(input_path)?;
    let mut visible: Forest = DMatrix::zeros(forest.nrows(), forest.ncols());
    for r in 0..forest.nrows() {
        for c in 0..forest.ncols() {
            let is_on_edge =
                r == 0 || r == (forest.nrows() - 1) || c == 0 || c == (forest.ncols() - 1);
            if is_on_edge {
                visible[(r, c)] = 1
            } else {
                // not on the edge, safe to look around
                let max_above = (0..=r - 1)
                    .map(|r_above| forest[(r_above, c)])
                    .max()
                    .expect("at least one tree above");
                let max_below = (r + 1..=forest.nrows() - 1)
                    .map(|r_below| forest[(r_below, c)])
                    .max()
                    .expect("at least one tree below");
                let max_left = (0..=c - 1)
                    .map(|c_left| forest[(r, c_left)])
                    .max()
                    .expect("at least one tree left");
                let max_right = (c + 1..=forest.ncols() - 1)
                    .map(|c_right| forest[(r, c_right)])
                    .max()
                    .expect("at least one tree right");
                let tree_height = forest[(r, c)];
                if tree_height > max_above
                    || tree_height > max_below
                    || tree_height > max_left
                    || tree_height > max_right
                {
                    visible[(r, c)] = 1;
                }
            }
        }
    }
    debug!("{:?}", visible);
    Ok(visible.into_owned().map(|e| e as i64).sum() as Score)
}

pub(crate) fn solve_day_8_challenge_2(input_path: &Path) -> anyhow::Result<Score> {
    let forest = parse_input(input_path)?;
    let mut scenic_score: Forest = DMatrix::zeros(forest.nrows(), forest.ncols());
    for r in 0..forest.nrows() {
        for c in 0..forest.ncols() {
            let is_on_edge =
                r == 0 || r == (forest.nrows() - 1) || c == 0 || c == (forest.ncols() - 1);
            if is_on_edge {
                scenic_score[(r, c)] = 0
            } else {
                // not on the edge, safe to look around
                let tree_height = forest[(r, c)];

                let mut view_distance_above = 0;
                for r_above in (0..=r - 1).rev() {
                    view_distance_above += 1;
                    if forest[(r_above, c)] >= tree_height {
                        break;
                    }
                }

                let mut view_distance_below = 0;
                for r_below in r + 1..=forest.nrows() - 1 {
                    view_distance_below += 1;
                    if forest[(r_below, c)] >= tree_height {
                        break;
                    }
                }

                let mut view_distance_left = 0;
                for c_left in (0..=c - 1).rev() {
                    view_distance_left += 1;
                    if forest[(r, c_left)] >= tree_height {
                        break;
                    }
                }

                let mut view_distance_right = 0;
                for c_right in c + 1..=forest.ncols() - 1 {
                    view_distance_right += 1;
                    if forest[(r, c_right)] >= tree_height {
                        break;
                    }
                }

                scenic_score[(r, c)] = view_distance_above
                    * view_distance_below
                    * view_distance_left
                    * view_distance_right;
                // if scenic_score[(r, c)] == scenic_score.max()  {
                //    debug!("tree at {} {} : {} = above {} * below {} * left {} * right {}", r, c, scenic_score[(r, c)], view_distance_above, view_distance_below, view_distance_left, view_distance_right);
                //}
            }
        }
    }
    // debug!("{:?}", scenic_score);
    Ok(scenic_score.into_owned().map(|e| e as i64).max() as Score)
}
