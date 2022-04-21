// Potential improvements:
//

use std::{collections::HashMap, error::Error};
use chrono::{NaiveDateTime, NaiveDate, Timelike};
use mediumvec::Vec32;


pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    
    let answer1 = part1(input_lines);
    // let answer2 = part2(input_lines);
    (format!("{}", answer1.0), format!("{}", answer1.1))
}

fn part1(input_lines: &[Vec<String>]) -> (usize, usize) {
    let sorted_lines = sort_times(input_lines).unwrap();
    let mut guard_sleeps = HashMap::new();
    let mut guard_in_question = String::new();
    let mut sleep_times = Vec::new();
    let mut wake_times = Vec::new();
    for i in sorted_lines {
        let time = i.0;
        let vec: Vec<String> = i.1.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        // println!("vec = {:#?}", vec);
        if vec[2] == "Guard" {
            //Finish off previous guard's schedule
            if guard_in_question != String::new() {
                let schedule = guard_sleeps.entry(guard_in_question.clone()).or_insert_with(|| vec![0; 60]);
                for j in 0..sleep_times.len() {
                    for i in sleep_times[j]..wake_times[j] {
                        schedule[i as usize] +=1;
                    }
                }
            }
            sleep_times = Vec::new();
            wake_times = Vec::new();
            guard_in_question = vec[3].clone();
            if !guard_sleeps.contains_key(&guard_in_question) {
                guard_sleeps.insert(guard_in_question.clone(), vec!(0;60));
            }
        }  
        else if vec[2] == "falls" {
            sleep_times.push(time.minute());    
        }
        else if vec[2] == "wakes" {
            wake_times.push(time.minute());
        } 
    }
    let key_with_max_value = &guard_sleeps.iter().max_by(|a_guard, b_guard| a_guard.1.iter().sum::<usize>().cmp(&b_guard.1.iter().sum())).map(|(k, v)| k);
    println!("{:#?}", guard_sleeps);
    println!("here: {:#?}", key_with_max_value.unwrap());
    let key: usize = key_with_max_value.unwrap().replace('#', "").parse().unwrap();
    let value = guard_sleeps.get(key_with_max_value.unwrap()).unwrap();
    let minute_of_most_sleep = max_index(value);
    let result1 = key * minute_of_most_sleep;

    let biggest_minute_hit = &guard_sleeps.iter().max_by(|a_guard, b_guard|a_guard.1.iter().max().cmp(&b_guard.1.iter().max())).map(|(k, v)| (k, v));
    let key: usize = biggest_minute_hit.unwrap().0.replace('#', "").parse().unwrap();
    let value = max_index(biggest_minute_hit.unwrap().1);
    let result2 = key *value;

    (result1, result2)
}

fn max_index(array: &[usize]) -> usize {
    let mut i = 0;

    for (j, &value) in array.iter().enumerate() {
        if value > array[i] {
            i = j;
        }
    }

    i
}

fn sort_times(input_lines: &[Vec<String>]) -> Result<Vec<(NaiveDateTime, String)>, String> {
    let mut sorted_lines = Vec::new();
    for i in &input_lines[0] {
        let words: Vec<&str> = i.split_whitespace().collect();
        // println!("{:#?}", words);
        let date_time = &words[0..2].join(" ").replace(&['[', '\"', ']'][..], "");
        // println!("{:#?}", date_time);
        let no_timezone = match NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M") {
            Ok(time) => time,
            Err(e) => return Err(String::from("couldn't parse time"))
        };
        sorted_lines.push((no_timezone, String::from(&i[..])));
        
    }
    sorted_lines.sort();
    // println!("{:#?}", sorted_lines);
    Ok(sorted_lines)
}
// fn part2(input_lines: &[Vec<String>]) -> String {
//     todo!
// }

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case04() {
        full_test(
"", // INPUT STRING
"0", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day04(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}
