use std::{fs};

fn main() {
    let all_moves: String = fs::read_to_string("./src/input.txt").expect("Unable to read file");
    let all_moves: Vec<&str> = all_moves.split('\n').collect();

    let moves: Vec<[char; 2]> = all_moves.iter().map(|f| [f.chars().into_iter().next().unwrap(), f.chars().into_iter().last().unwrap()]).collect();

    let mut score: u32 = 0;

    for m in moves {
        let my_move = m[1];
        let opp_move = m[0];

        match my_move {
            'X' => {
                score += 1;
                match opp_move {
                    'A' => score += 3,
                    'C' => score += 6,
                    _ => {}
                }
            },
            'Y' => {
                score += 2;
                match opp_move {
                    'A' => score += 6,
                    'B' => score += 3,
                    _ => {}
                }
            },
            'Z' => {
                score += 3;
                match opp_move {
                    'B' => score += 6,
                    'C' => score += 3,
                    _ => {}
                }
            },
            _ => {}
        }
    }

    println!("==================");
    println!("Solution is: {}", score);
    println!("==================");
}