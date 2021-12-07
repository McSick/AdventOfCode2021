use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::{self, BufRead};
use std::path::Path;

const FILENAME: &str = "./input.txt";
fn main() {
    let (min_max, crabs) = get_input();
    let least_fuel = move_crabs(min_max, crabs);
    println!("Least fuel: {}", least_fuel);

}
fn move_crabs(min_max: (i32, i32), crabs: HashMap<i32, i32>) -> i32 {
    let mut smallest = i32::MAX;
    for i in min_max.0..(min_max.1 + 1) {
        let mut total_fuel = 0;
        for (key, val) in crabs.iter() {
            let dist = (key - i).abs();
            //Part 1
            //let fuelconsume = dist * val;

            //Part 2
            //Triangular number formula
            let fuelconsume = (dist + 1) * dist / 2 * val;
            total_fuel += fuelconsume;
        }
        if total_fuel < smallest {
            smallest = total_fuel;
        }
    }
    smallest
}
fn get_input() -> ((i32, i32), HashMap<i32, i32>) {
    let mut crabs = HashMap::new();
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    if let Ok(lines) = read_lines(FILENAME) {
        let mut input_iter: Lines<BufReader<File>> = lines.into_iter();
        let string = input_iter.next().unwrap().ok().unwrap();
        let str_iter = string.split(",");
        for numstr in str_iter {
            let num = numstr.parse::<i32>().unwrap();
            *crabs.entry(num).or_insert(0) += 1;

            if num < min {
                min = num;
            }
            if num > max {
                max = num
            }
        }
    }
    ((min, max), crabs)
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
