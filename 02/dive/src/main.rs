use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Nav {
    command:String,
    distance:u32
}
fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let navcommands: Vec<Nav> = lines.into_iter().map(|l| {
            l.ok().and_then(|s| { 
                let chunks: Vec<_> = s.split_whitespace().collect();
                
                return Some(Nav{ command: chunks[0].parse().unwrap(), distance:chunks[1].parse().unwrap()})
            }
        ).unwrap()
        }).collect();
        dive1(&navcommands);
        dive2(&navcommands);

    }

    
}

fn dive1(navcommands: &Vec<Nav>) -> () {
    let mut horizontal = 0;
    let mut depth = 0;
    for i in 0..navcommands.len() {
        match navcommands[i].command {
            _ if navcommands[i].command == "up" => depth -= navcommands[i].distance,
            _ if navcommands[i].command == "down" => depth += navcommands[i].distance,
            _ if navcommands[i].command == "forward" => horizontal += navcommands[i].distance,
            _ => println!("Oh no")
        } 
    }
    println!("Horizontal: {} Depth: {} Mul: {}", horizontal, depth, horizontal*depth);
}
fn dive2(navcommands: &Vec<Nav>) -> () {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for i in 0..navcommands.len() {
        match navcommands[i].command {
            _ if navcommands[i].command == "up" => { 
                aim -= navcommands[i].distance;
            },
            _ if navcommands[i].command == "down" => { 
                aim += navcommands[i].distance;
            },
            _ if navcommands[i].command == "forward" => {
                horizontal += navcommands[i].distance;
                depth += aim * navcommands[i].distance;
            },
            _ => println!("Oh no")
        } 
    }
    println!("Horizontal: {} Depth: {} Mul: {}", horizontal, depth, horizontal*depth);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}