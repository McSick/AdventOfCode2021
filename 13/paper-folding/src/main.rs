use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
#[derive(Debug, PartialEq)]
enum FoldDirection {
    X,
    Y
}
fn main() {
    part1();
    part2();
}

fn part1() {
    let (coords, commands) = get_input("input.txt");
    let p1 = run_n_commands(coords, commands, 1);
    println!("part1: {:?}", p1.len());
}

fn part2() {
    let (coords, commands) = get_input("input.txt");
    let num_commands = commands.len();
    let p2 = run_n_commands(coords, commands, num_commands);
    pretty_print(p2);
}
fn pretty_print(coords: HashSet<(usize,usize)>) {

    let x_max = coords.iter().fold(0, |acc, &coor| cmp::max(acc, coor.0)) + 1;
    let y_max = coords.iter().fold(0, |acc, &coor| cmp::max(acc, coor.1)) + 1;
    let mut final_arr = vec![vec!['\u{A0}'; x_max]; y_max];
    for coord in coords {
        final_arr[coord.1][coord.0] = '\u{25A0}';
    }
    for y in 0..y_max {
        for x in 0..x_max {
            print!("{}", final_arr[y][x]);
        }
        println!("");
    }
}
fn run_n_commands(mut coords: HashSet<(usize,usize)>, commands: Vec<(FoldDirection, usize)>, n:usize) -> HashSet<(usize,usize)> {
    for i in 0..n {
        let command = &commands[i];
        coords = HashSet::<(usize,usize)>::from_iter(coords.into_iter().map(|c|  { 
            match command.0 {
                FoldDirection::X => { 
                    if c.0 > command.1 {
                        return (command.1*2 - c.0, c.1);
                    } else {
                        return c;
                    }
                },
                FoldDirection::Y => {
                    if c.1 > command.1 {
                        return (c.0, 2*command.1 - c.1);
                    } else {
                        return c;
                    }
                }
            }
        }));  
    }
    coords
}

#[test]
fn test_part1() {
    let (coords, commands) = get_input("./test.txt");
    let result = run_n_commands(coords, commands, 1);
    assert_eq!(result.len(), 17);
}
#[test]
fn parse_input() {
    let (coords, commands) = get_input("./test.txt");
    assert!(coords.contains(&(9,10)));
    assert_eq!(commands[0].0, FoldDirection::Y);
    assert_eq!(commands[0].1, 7);
}

fn get_input(filename: &str) -> (HashSet<(usize,usize)>, Vec<(FoldDirection, usize)>) {
    let mut coords = HashSet::<(usize,usize)>::new();
    let mut commands = Vec::<(FoldDirection, usize)>::new();
    if let Ok(lines) = read_lines(filename) {
        for someline in lines {
            if let Ok(line) = someline {
                if line.starts_with("f") {
                    match line.split_once("=") {
                        Some((command, point)) => {
                            if command.ends_with("x") {
                                commands.push((FoldDirection::X, point.parse().ok().unwrap()))
                            } else {
                                commands.push((FoldDirection::Y, point.parse().ok().unwrap()))
                            }
                        },
                        None => ()
                    }
                } else {
                    match line.split_once(",") {
                        Some((x_str, y_str)) => {
                            coords.insert((x_str.parse().ok().unwrap(), y_str.parse().ok().unwrap()));
                        },
                        None => ()
                    }
                }
            }
        }
    }
    (coords, commands)
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}