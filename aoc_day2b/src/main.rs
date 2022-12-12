use std::{fs};

fn main() {
    let all_moves: String = fs::read_to_string("../aoc_day2/src/input.txt").expect("Unable to read file");
    let all_moves: Vec<&str> = all_moves.split('\n').collect();

    let moves: Vec<[char; 2]> = all_moves.iter().map(|f| [f.chars().into_iter().next().unwrap(), f.chars().into_iter().last().unwrap()]).collect();

    let mut score: u32 = 0;

    for m in moves {
        let my_move = m[1];
        let opp_move = m[0];

        match my_move {
            'X' => {
                score += 0;
                match opp_move {
                    'A' => score += 3,
                    'C' => score += 2,
                    _ => score += 1
                }
            },
            'Y' => {
                score += 3;
                match opp_move {
                    'A' => score += 1,
                    'B' => score += 2,
                    _ => {score += 3}
                }
            },
            'Z' => {
                score += 6;
                match opp_move {
                    'B' => score += 3,
                    'C' => score += 1,
                    _ => {score += 2}
                }
            },
            _ => {}
        }
    }

    println!("==================");
    println!("Solution is: {}", score);
    println!("==================");
}