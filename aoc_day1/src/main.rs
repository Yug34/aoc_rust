use std::{fs};

fn main() {
    let all_calories: String = fs::read_to_string("./src/input.txt").expect("Unable to read file");
    let elf_calorie: Vec<&str> = all_calories.split("\n\n").collect();

    let mut max_calories: u32 = 0;

    for cal_items_in_elf in elf_calorie {
        let mut current_calories: u32 = 0;
        
        for cal_item in cal_items_in_elf.split("\n") {
            match cal_item.to_string().parse::<u32>() {
                Ok(n) => current_calories += n,
                Err(_) => {}
            }

            if current_calories > max_calories {
                max_calories = current_calories;
            }
        }
    }

    println!("==================");
    println!("Solution is: {}", max_calories);
    println!("==================");
}