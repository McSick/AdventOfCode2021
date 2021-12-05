use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::{self, BufRead};
use std::path::Path;
const BOARDSIZE: usize = 1000;
fn main() {
    let mut vents: Vec<Vent> = get_input();
    let mut board: Vec<Vec<u16>> = vec![vec![0; BOARDSIZE]; BOARDSIZE];

    count_vents(vents, board);
}
struct Vent {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}
fn count_vents(vents: Vec<Vent>, mut board: Vec<Vec<u16>>) {
    for vent in vents {
        if vent.x1 == vent.x2 {
            traverse_y(vent, &mut board);
        } else if vent.y1 == vent.y2 {
            traverse_x(vent, &mut board);
        } else {
            traverse_diag(vent, &mut board);
        }
    }
    count_intersectiosn(board);
}
fn count_intersectiosn(board: Vec<Vec<u16>>) {
    let mut count = 0;
    for i in 0..BOARDSIZE {
        for j in 0..BOARDSIZE {
            if board[i][j] > 1 {
                count += 1;
            }
        }
    }
    println!("Intersections: {}", count);
}

fn traverse_diag(vent: Vent, board: &mut Vec<Vec<u16>>) {
    let mut curr = 0;
    let mut max = 0;
    let mut curry = 0;
    let mut incr: i16 = 0;
    if vent.x1 < vent.x2 {
        curr = vent.x1 as i16;
        max = vent.x2 as i16;
        curry = vent.y1 as i16;
        incr = if vent.y1 < vent.y2 { 1 } else { -1 };
    } else {
        curr = vent.x2 as i16;
        max = vent.x1 as i16;
        curry = vent.y2 as i16;
        incr = if vent.y1 > vent.y2 { 1 } else { -1 };
    };

    while curr <= max {
        board[curr as usize][curry as usize] += 1;
        curr += 1;
        curry += incr;
    }
}
fn traverse_y(vent: Vent, board: &mut Vec<Vec<u16>>) {
    let mut curr = if vent.y1 > vent.y2 { vent.y2 } else { vent.y1 };
    let max = if vent.y1 > vent.y2 { vent.y1 } else { vent.y2 };
    while curr <= max {
        board[vent.x1][curr] += 1;
        curr += 1;
    }
}
fn traverse_x(vent: Vent, board: &mut Vec<Vec<u16>>) {
    let mut curr = if vent.x1 > vent.x2 { vent.x2 } else { vent.x1 };
    let max = if vent.x1 > vent.x2 { vent.x1 } else { vent.x2 };

    while curr <= max {
        board[curr][vent.y1] += 1;
        curr += 1;
    }
}
fn get_input() -> Vec<Vent> {
    let mut vents: Vec<Vent> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        let mut input_iter: Lines<BufReader<File>> = lines.into_iter();
        while let Some(line) = input_iter.next() {
            let parsed: Vec<Vec<i32>> = line
                .unwrap()
                .split(" -> ")
                .map(|s| {
                    s.to_string()
                        .split(",")
                        .map(|n| n.parse().ok().unwrap())
                        .collect()
                })
                .collect();
            vents.push(Vent {
                x1: parsed[0][0] as usize,
                y1: parsed[0][1] as usize,
                x2: parsed[1][0] as usize,
                y2: parsed[1][1] as usize,
            });
        }
    }
    (vents)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
