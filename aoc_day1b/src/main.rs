use std::{fs};

fn main() {
    let all_calories: String = fs::read_to_string("../aoc_day1/src/input.txt").expect("Unable to read file");
    let elf_calorie: Vec<&str> = all_calories.split("\n\n").collect();

    let mut max_calories: [u32; 3] = [0, 0, 0];

    for cal_items_in_elf in elf_calorie {
        let mut current_calories: u32 = 0;
        
        for cal_item in cal_items_in_elf.split("\n") {
            match cal_item.to_string().parse::<u32>() {
                Ok(n) => current_calories += n,
                Err(_) => {}
            }

            if current_calories > max_calories[0] {
                max_calories[0] = current_calories;
                max_calories.sort();
            }
        }
    }

    let final_sum: u32 = max_calories.iter().sum();

    println!("===================");
    println!("Solution is: {}", final_sum);
    println!("===================");
}