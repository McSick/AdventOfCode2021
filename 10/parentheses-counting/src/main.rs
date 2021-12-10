use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let (p1, p2) = count_paren(lines);
        println!("p1: {} p2: {}", p1, p2);
    }
}

fn count_paren(lines: io::Lines<io::BufReader<File>>) -> (i32, i64) {
    let mut part1 = 0;
    let mut scores = Vec::new();
    for someline in lines {
        if let Ok(line) = someline {
            let (invalid_cost, validstack) = validate_line(line);
            part1 += invalid_cost;
            if let Some(stack) = validstack {
                scores.push(complete_the_sequence(stack));
            }
        }
    }
    scores.sort();
    let halfindex = scores.len() / 2 as usize;
    let part2 = scores[halfindex];
    (part1, part2)
}
fn complete_the_sequence(mut stack: Vec<char>) -> i64 {
    let mut score = 0;
    while let Some(character) = stack.pop() {
        match character {
            '(' => score = score * 5 + 1,
            '[' => score = score * 5 + 2,
            '{' => score = score * 5 + 3,
            '<' => score = score * 5 + 4,
            _ => panic!("Invalid character in stack"),
        }
    }
    score
}
fn validate_line(line: String) -> (i32, Option<Vec<char>>) {
    let mut stack = Vec::new();
    for character in line.chars() {
        let mut invalid_cost = 0;
        match character {
            '(' => stack.push(character),
            ')' => invalid_cost += get_points_if_invalid(&mut stack, character),
            '[' => stack.push(character),
            ']' => invalid_cost += get_points_if_invalid(&mut stack, character),
            '{' => stack.push(character),
            '}' => invalid_cost += get_points_if_invalid(&mut stack, character),
            '<' => stack.push(character),
            '>' => invalid_cost += get_points_if_invalid(&mut stack, character),
            _ => panic!("Unexpected character: {}", character),
        }
        if invalid_cost > 0 {
            return (invalid_cost, None);
        }
    }
    (0, Some(stack))
}

fn get_points_if_invalid(stack: &mut Vec<char>, elem: char) -> i32 {
    if let Some(top) = stack.pop() {
        match elem {
            ')' => {
                if top != '(' {
                    return 3;
                }
            }
            ']' => {
                if top != '[' {
                    return 57;
                }
            }
            '}' => {
                if top != '{' {
                    return 1197;
                }
            }
            '>' => {
                if top != '<' {
                    return 25137;
                }
            }
            _ => (),
        }
    }
    0
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test_complete_the_sequence() {
    let (_, validstack) = validate_line("[({(<(())[]>[[{[]{<()<>>".to_string());
    if let Some(stack) = validstack {
        let part2 = complete_the_sequence(stack);
        assert_eq!(part2, 288957);
    }
}

#[test]
fn test_validate_line() {
    let (count, _) = validate_line("{([(<{}[<>[]}>{[]{[(<()>".to_string());
    assert_eq!(count, 1197);
    let (count, _) = validate_line("[[<[([]))<([[{}[[()]]]".to_string());
    assert_eq!(count, 3);
    let (count, _) = validate_line("[{[{({}]{}}([{[{{{}}([]".to_string());
    assert_eq!(count, 57);
    let (count, _) = validate_line("[<(<(<(<{}))><([]([]()".to_string());
    assert_eq!(count, 3);
    let (count, _) = validate_line("<{([([[(<>()){}]>(<<{{".to_string());
    assert_eq!(count, 25137);
    let (count, _) = validate_line("(({[<>]}))".to_string());
    assert_eq!(count, 0);
}
#[test]
fn test_count_paren() {
    if let Ok(lines) = read_lines("simple.txt") {
        let (p1, p2) = count_paren(lines);
        assert_eq!(p1, 26397);
        assert_eq!(p2, 288957);
    } else {
        panic!("Couldn't load file");
    }
}
