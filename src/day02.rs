// Potential improvements:
//

use std::{collections::{HashMap, HashSet}};



pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    
    let answer1 = day02_part1(input_lines);
    let answer2 = day02_part2(input_lines);
    (answer1, answer2)
    
}

pub fn day02_part1(input_lines: &[Vec<String>]) -> String {
    let mut contains_double = 0;
    let mut contains_triple = 0;
    for a in &input_lines[0] {
        println!("{:#?}", a);
        let mut letter_count = HashMap::new();
        for letter in a.chars() {
            *letter_count.entry(letter).or_insert(0) += 1;
        };
        println!("{:#?}", letter_count);
        contains_double += if letter_count.values().collect::<HashSet<&i32>>().contains(&2) {1} else {0};
        contains_triple += if letter_count.values().collect::<HashSet<&i32>>().contains(&3) {1} else {0};

    }
    return format!("{}", contains_double*contains_triple)
}

pub fn day02_part2(input_lines: &[Vec<String>]) -> String {
    let  mut position_letter_hash_hash = HashMap::new();
    for a in &input_lines[0] {
        for i in 1..=input_lines[0][0].len() {
            let position_hash = position_letter_hash_hash.entry(i ).or_insert_with(HashMap::new);
            position_hash.entry(a.chars().nth(i).unwrap()).or_insert_with(|| vec![a]).push(a);
        }
    }
    
    return format!("hi")
}

#[cfg(test)]
mod tests {
    use super::day02_part1; 
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        assert_eq!(day02_part1(&load_input("abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab")), "12");
    }
}