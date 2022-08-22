use std::{vec, collections::HashMap};
use std::num::Wrapping;

pub fn day12(input_lines: &[Vec<String>]) -> (String, String) {
    let rules = rules(input_lines);
    //part1(input_lines, rules);
    let answer1 = part1(input_lines, rules);
    let answer2 = 0;
    (format!("{:#?}", answer1), format!("{}", answer2))
}

fn part1(input_lines: &[Vec<String>], rules: HashMap<&str, &str>) -> usize {
    let initial_state = input_lines[0][0].replace("initial state: ", "");
    let mut state_as_string = initial_state;
    let mut pot_zero_index = 0;
    let years = 2147483647;
    for year in 0..years {
        let index_of_first_hash = state_as_string.find("#");
        if index_of_first_hash.unwrap() <= 4 {
            for i in 0..(4-index_of_first_hash.unwrap()) {
                // println!("round {}, adding dot to left hash index was {}", year, index_of_first_hash.unwrap());
                state_as_string = ".".to_owned() + &state_as_string;
                pot_zero_index += 1;
            }
            // println!("pot zero index: {}", pot_zero_index);
        }
        let index_of_last_hash = state_as_string.rfind("#");
        for i in 0..(5 -(state_as_string.len() - index_of_last_hash.unwrap())) {
            // println!("round {}, adding dot to right", year);
            state_as_string = state_as_string + ".";
        }
        let mut new_state = "..".to_string();
        
        for index in 2..state_as_string.len()-2 {
            let fivesome = &state_as_string[index-2..index+3];
            // println!("fivesome: {}", fivesome);
            let &new_plant = rules.get(fivesome).unwrap();
            new_state += new_plant;
        }
        // println!("{:#?}", new_state);
        if state_as_string == new_state.clone() + ".." {
            break
        }
        else {
            println!
            state_as_string = new_state + "..";
        // println!{"{}", state_as_string};
        }
        
    }
    let mut count = 0;
    let final_list: Vec<char> = state_as_string.chars().collect();
    for i in 0..state_as_string.len() {
        if final_list[i] == '#' {
            count += (Wrapping(i)- Wrapping(pot_zero_index)).0;
        }
    }
    println!("count: {}", count);
    count
}

fn rules(input_lines: &[Vec<String>]) -> HashMap<&str, &str> {
    let mut rules: HashMap<&str, &str> = HashMap::new();
    for i in 1..input_lines[0].len() {
        let mut rule = input_lines[0][i].split(" => ");
        rules.insert(rule.next().unwrap(), rule.next().unwrap());
    }  
    // println!("{:#?}", rules);
    rules
}


#[cfg(test)]
mod tests {
    use super::day12;
    use crate::utils::load_input;

    #[test]
    fn check_day12_case01() {
        full_test(
            "initial state: #
.##.. => .
..##. => #
.#..# => #
.#.#. => .
..#.. => #
###.. => #
##..# => .
##... => #
#.### => #
.##.# => #
#.... => .
###.# => .
..... => .
.#... => #
....# => .
#.#.. => .
...#. => #
#...# => .
##.#. => .
.#.## => #
..#.# => #
#.#.# => .
.#### => .
##### => .
..### => .
...## => .
#..## => .
#.##. => .
#..#. => #
.###. => #
##.## => #
####. => .", // INPUT STRING
            "57", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day12(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}