use anyhow::anyhow;
use graph::prelude::{DirectedCsrGraph, DirectedNeighbors, GraphBuilder};
use itertools::Itertools;
use log::debug;
use nalgebra::{DMatrix, RowDVector};
use pathfinding::prelude::astar;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Score = u64;
type Heightmap = DMatrix<i32>;

pub(crate) fn parse_input(input_path: &Path) -> anyhow::Result<Heightmap> {
    let input_file = File::open(input_path)?;
    let buffer_reader = BufReader::new(input_file);
    let rows: Vec<RowDVector<i32>> = buffer_reader
        .lines()
        .flat_map(|line| {
            if let Ok(line) = line {
                let heights = line.chars().map(|c| c as i32).into_iter();
                Some(RowDVector::from_iterator(line.len(), heights))
            } else {
                None
            }
        })
        .collect();
    let heightmap: Heightmap = DMatrix::from_rows(rows.as_slice());
    Ok(heightmap)
}

pub(crate) fn solve_day_12_challenge_1(input_path: &Path) -> anyhow::Result<Score> {
    let heightmap = parse_input(input_path)?;

    fn reindex(r: i32, c: i32, ncols: i32) -> i32 {
        c + r * ncols
    }

    fn deindex(index: i32, ncols: i32) -> (i32, i32) {
        (index / ncols, index % ncols)
    }

    let mut edges: Vec<(i32, i32)> = vec![];
    let mut start: Option<(i32, i32)> = None;
    let mut end: Option<(i32, i32)> = None;
    for r in 0..heightmap.nrows() {
        for c in 0..heightmap.ncols() {
            let current = heightmap[(r, c)];
            if current == 'S' as i32 {
                start = Some((r as i32, c as i32))
            }
            if current == 'E' as i32 {
                end = Some((r as i32, c as i32))
            }

            for (dr, dc) in [(-1i32, 0i32), (1i32, 0i32), (0i32, -1i32), (0i32, 1i32)] {
                if (r as i32 + dr) >= 0
                    && (r as i32 + dr < heightmap.nrows() as i32)
                    && (c as i32 + dc >= 0)
                    && (c as i32 + dc < heightmap.ncols() as i32)
                {
                    let other = heightmap[(r + dr as usize, c + dc as usize)];

                    let current = if current == 'S' as i32 {
                        'a' as i32
                    } else {
                        current
                    };
                    let other = if other == 'E' as i32 {
                        'z' as i32
                    } else {
                        other
                    };

                    if current + 1 >= other {
                        edges.push((
                            reindex(r as i32, c as i32, heightmap.ncols() as i32),
                            reindex(
                                (r as i32 + dr) as i32,
                                (c as i32 + dc) as i32,
                                heightmap.ncols() as i32,
                            ),
                        ));
                    }
                }
            }
        }
    }
    let graph: DirectedCsrGraph<i32> = GraphBuilder::new().edges(edges).build();
    let start = start.expect("start");
    let end = end.expect("end");
    let start = reindex(start.0, start.1, heightmap.ncols() as i32);
    let end = reindex(end.0, end.1, heightmap.ncols() as i32);
    let path = astar(
        &start,
        |&index| {
            graph
                .out_neighbors(index)
                .map(|p| (*p, 1))
                .collect::<Vec<(i32, u32)>>()
        },
        |p| {
            let (rp, cp) = deindex(*p, heightmap.ncols() as i32);
            let (ro, co) = deindex(end, heightmap.ncols() as i32);
            (rp.abs_diff(ro) + cp.abs_diff(co)) as u32
        },
        |p| *p == end,
    );
    if let Some((path, cost)) = path {
        let mut matrix = DMatrix::<i32>::zeros(heightmap.nrows(), heightmap.ncols());
        for index in path.iter() {
            let (r, c) = deindex(*index, heightmap.ncols() as i32);
            matrix[(r as usize, c as usize)] = 1;
            debug!(
                "{:?} {:?}",
                (r, c),
                heightmap[(r as usize, c as usize)] as u8 as char
            );
        }
        for row in matrix.row_iter() {
            println!("{:?}", row.iter().join(""));
        }
        Ok(cost as u64)
    } else {
        Err(anyhow!("no path found"))
    }
}

pub(crate) fn solve_day_12_challenge_2(input_path: &Path) -> anyhow::Result<Score> {
    let heightmap = parse_input(input_path)?;

    fn reindex(r: i32, c: i32, ncols: i32) -> i32 {
        c + r * ncols
    }

    fn deindex(index: i32, ncols: i32) -> (i32, i32) {
        (index / ncols, index % ncols)
    }

    let mut edges: Vec<(i32, i32)> = vec![];
    let mut starts: Vec<(i32, i32)> = vec![];
    let mut end: Option<(i32, i32)> = None;
    for r in 0..heightmap.nrows() {
        for c in 0..heightmap.ncols() {
            let current = heightmap[(r, c)];
            if current == 'S' as i32 || current == 'a' as i32 {
                starts.push((r as i32, c as i32));
            }
            if current == 'E' as i32 {
                end = Some((r as i32, c as i32))
            }

            for (dr, dc) in [(-1i32, 0i32), (1i32, 0i32), (0i32, -1i32), (0i32, 1i32)] {
                if (r as i32 + dr) >= 0
                    && (r as i32 + dr < heightmap.nrows() as i32)
                    && (c as i32 + dc >= 0)
                    && (c as i32 + dc < heightmap.ncols() as i32)
                {
                    let other = heightmap[(r + dr as usize, c + dc as usize)];

                    let current = if current == 'S' as i32 {
                        'a' as i32
                    } else {
                        current
                    };
                    let other = if other == 'E' as i32 {
                        'z' as i32
                    } else {
                        other
                    };

                    if current + 1 >= other {
                        edges.push((
                            reindex(r as i32, c as i32, heightmap.ncols() as i32),
                            reindex(
                                (r as i32 + dr) as i32,
                                (c as i32 + dc) as i32,
                                heightmap.ncols() as i32,
                            ),
                        ));
                    }
                }
            }
        }
    }
    let graph: DirectedCsrGraph<i32> = GraphBuilder::new().edges(edges).build();
    let shortest_path = starts
        .iter()
        .filter_map(|start| {
            let end = end.expect("end");
            let start = reindex(start.0, start.1, heightmap.ncols() as i32);
            let end = reindex(end.0, end.1, heightmap.ncols() as i32);
            astar(
                &start,
                |&index| {
                    graph
                        .out_neighbors(index)
                        .map(|p| (*p, 1))
                        .collect::<Vec<(i32, u32)>>()
                },
                |p| {
                    let (rp, cp) = deindex(*p, heightmap.ncols() as i32);
                    let (ro, co) = deindex(end, heightmap.ncols() as i32);
                    (rp.abs_diff(ro) + cp.abs_diff(co)) as u32
                },
                |p| *p == end,
            )
        })
        .map(|(_path, cost)| cost)
        .min();

    if let Some(shortest_path) = shortest_path {
        Ok(shortest_path as u64)
    } else {
        Err(anyhow!("no path found"))
    }
}
