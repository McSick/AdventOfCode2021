use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::Iterator;
const FILENAME:&str = "./input.txt";
const DAYS:i32 = 256;
const CYCLE: usize = 9;
fn main() {
    let fish = get_input();
    let number_of_fish = simulate_fish(fish);
    println!("Fish Count: {:?}", number_of_fish);
}
fn simulate_fish(mut fish: [u64; CYCLE]) -> u64 {
    for _i in 0..DAYS {
        let restart = fish[0];
        for j in 1..CYCLE {
            fish[j-1] = fish[j];
        }
        fish[CYCLE - 3] += restart;
        fish[CYCLE - 1] = restart;
    }
    return fish.iter().sum::<u64>();

}
fn get_input() -> ([u64; CYCLE]) {
    let mut fish: [u64; CYCLE] = [0; CYCLE];
    if let Ok(lines) = read_lines(FILENAME) {
        let mut input_iter: Lines<BufReader<File>> = lines.into_iter();
        let string = input_iter.next().unwrap().ok().unwrap();
        let str_iter = string.split(",");
        for numstr in str_iter {
            fish[numstr.parse::<usize>().unwrap()] += 1;
        }
    }
    fish
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}