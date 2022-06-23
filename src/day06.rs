// Potential improvements:
//

pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    part1(input_lines);
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(input_lines: &[Vec<String>]) {
    let mut mines: Vec<Vec<i32>> = Vec::new();
    for i in input_lines[0].iter() {
        let coordinates: Vec<i32> = i.split(", ").map(|word| i32::from_str_radix(word, 32).unwrap()).collect();
        mines.push(coordinates)
    }
    println!("mines: {:#?}", mines);
    let left = mines[0][0];
    let right = mines[0][0];
    let up = mines[0][1];
    let down = mines[0][1];
    for mine in mines {
        if mine[0] < 2 {
            return
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case06() {
        full_test(
"", // INPUT STRING
"0", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day06(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}
