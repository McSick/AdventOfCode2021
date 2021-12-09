use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::{self, BufRead};
use std::path::Path;
struct BingoBoard {
    total: i32,
    numbers: HashMap<i32, (i32, i32)>,
    rows: [usize; 5],
    cols: [usize; 5],
    has_won: bool,
}
impl BingoBoard {
    fn new() -> Self {
        BingoBoard {
            total: 0,
            numbers: HashMap::new(),
            rows: [0; 5],
            cols: [0; 5],
            has_won: false,
        }
    }
    fn add_number(&mut self, num: i32, r: i32, c: i32) {
        self.numbers.insert(num, (r, c));
        self.total += num;
    }
    fn check_rows_and_cols(&mut self, row_num: (usize, usize)) -> bool {
        self.rows[row_num.0] += 1;
        self.cols[row_num.1] += 1;
        if self.rows[row_num.0] == 5 || self.cols[row_num.1] == 5 {
            self.has_won = true;
            return true;
        } else {
            false
        }
    }
    fn match_number(&mut self, num: i32) -> bool {
        let row_num = *(self.numbers.get(&num).unwrap_or(&(-1, -1)));
        if row_num.0 != -1 && row_num.1 != -1 {
            self.total -= num;
            self.check_rows_and_cols((row_num.0 as usize, row_num.1 as usize))
        } else {
            false
        }
    }
}
fn main() {
    let (nums, bingoboards) = get_input();
    let (part1, part2) = simulate_bingo(nums, bingoboards);
    println!("Answer 1: {} Answer 2: {}", part1, part2);
}
fn simulate_bingo(nums: Vec<i32>, mut bingoboards: Vec<BingoBoard>) -> (i32, i32) {
    let mut firstwin: i32 = -1;
    let mut lastwin: i32 = -1;
    for num in nums {
        for board in &mut bingoboards {
            let didwin = board.match_number(num);
            if didwin && firstwin < 0 {
                firstwin = board.total * num;
            }
        }
        if bingoboards.len() == 1 && bingoboards[0].has_won {
            lastwin = bingoboards[0].total * num;
            break;
        }
        bingoboards.retain(|board| !board.has_won);
    }
    (firstwin, lastwin)
}
fn get_input() -> (Vec<i32>, Vec<BingoBoard>) {
    let mut bingoboards: Vec<BingoBoard> = Vec::new();
    let mut r_nums: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        let mut input_iter: Lines<BufReader<File>> = lines.into_iter();
        r_nums = input_iter
            .next()
            .unwrap()
            .ok()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        while let Some(_blank_line) = input_iter.next() {
            let mut board: BingoBoard = BingoBoard::new();
            for i in 0..5 {
                if let Some(row) = input_iter.next() {
                    let new_row: Vec<i32> = row
                        .ok()
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.parse().ok().unwrap())
                        .collect();
                    for j in 0..5 {
                        board.add_number(new_row[j], i, j as i32);
                    }
                }
            }

            bingoboards.push(board);
        }
    }
    (r_nums, bingoboards)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
