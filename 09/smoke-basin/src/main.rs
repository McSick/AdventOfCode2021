use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
struct Loc {
    has_visted: bool,
    height: u8,
}
fn risk_level(heightmap: Vec<Vec<Loc>>) -> (u32, u32) {
    let mut risk_level = 0;
    let mut basin_count = 1;
    let (low_points, mut basins) = get_low_points(heightmap);
    for point in low_points {
        risk_level += point as u32 + 1;
    }
    basins.sort_by(|a, b| b.cmp(a));
    for i in 0..3 {
        basin_count *= basins[i];
    }
    (risk_level, basin_count)
}
fn get_low_points(mut heightmap: Vec<Vec<Loc>>) -> (Vec<u8>, Vec<u32>) {
    let mut lows = Vec::<u8>::new();
    let mut basins = Vec::<u32>::new();
    let rowlen = heightmap.len();
    for i in 0..rowlen {
        let collen = heightmap[i].len();
        for j in 0..collen {
            let mut neighbors = Vec::<u8>::new();
            if i > 0 {
                neighbors.push(heightmap[i - 1][j].height);
            }
            if i < (rowlen - 1) {
                neighbors.push(heightmap[i + 1][j].height);
            }
            if j > 0 {
                neighbors.push(heightmap[i][j - 1].height);
            }
            if j < (collen - 1) {
                neighbors.push(heightmap[i][j + 1].height);
            }
            if is_lower(heightmap[i][j].height, neighbors) {
                lows.push(heightmap[i][j].height);
                let count = climb_to_9((i, j), (rowlen, collen), &mut heightmap);
                basins.push(count);
            }
        }
    }
    (lows, basins)
}
fn is_lower(height: u8, neighbors: Vec<u8>) -> bool {
    for neighbor in neighbors {
        if neighbor <= height {
            return false;
        }
    }
    true
}
fn climb_to_9(
    indexes: (usize, usize),
    max_index: (usize, usize),
    heightmap: &mut Vec<Vec<Loc>>,
) -> u32 {
    let mut count_some_neighbors = 0;
    heightmap[indexes.0][indexes.1].has_visted = true;
    if heightmap[indexes.0][indexes.1].height < 9 {
        count_some_neighbors += 1;
    }
    if indexes.0 > 0
        && !heightmap[indexes.0 - 1][indexes.1].has_visted
        && heightmap[indexes.0 - 1][indexes.1].height < 9
    {
        count_some_neighbors += climb_to_9((indexes.0 - 1, indexes.1), max_index, heightmap);
    }
    if indexes.0 < (max_index.0 - 1)
        && !heightmap[indexes.0 + 1][indexes.1].has_visted
        && heightmap[indexes.0 + 1][indexes.1].height < 9
    {
        count_some_neighbors += climb_to_9((indexes.0 + 1, indexes.1), max_index, heightmap);
    }
    if indexes.1 > 0
        && !heightmap[indexes.0][indexes.1 - 1].has_visted
        && heightmap[indexes.0][indexes.1 - 1].height < 9
    {
        count_some_neighbors += climb_to_9((indexes.0, indexes.1 - 1), max_index, heightmap);
    }
    if indexes.1 < (max_index.1 - 1)
        && !heightmap[indexes.0][indexes.1 + 1].has_visted
        && heightmap[indexes.0][indexes.1 + 1].height < 9
    {
        count_some_neighbors += climb_to_9((indexes.0, indexes.1 + 1), max_index, heightmap);
    }

    count_some_neighbors
}

fn main() {
    let heightmap = get_input("input.txt");
    let (risk, basin) = risk_level(heightmap);
    println!("Risk: {}, Basin: {}", risk, basin);
}

fn get_input(filename: &str) -> Vec<Vec<Loc>> {
    let mut heightmap = Vec::<Vec<Loc>>::new();
    if let Ok(lines) = read_lines(filename) {
        for someline in lines {
            if let Ok(line) = someline {
                let mut vecline = Vec::<Loc>::new();
                for digit in line.chars() {
                    vecline.push(Loc {
                        has_visted: false,
                        height: digit.to_digit(10).unwrap() as u8,
                    });
                }
                heightmap.push(vecline);
            }
        }
    }
    heightmap
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test_find_risk_level() {
    let vents = get_input("simple.txt");
    let (r, b) = risk_level(vents);
    assert_eq!(r, 15, "Risk level should be 15, answer was `{}`", r);
    assert_eq!(b, 1134, "Basin should be 1134, answer was `{}`", b);
}
