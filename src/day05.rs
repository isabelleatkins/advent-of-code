// Potential improvements:
//

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    
    let answer1 = part1(input_lines);
    (format!("{:#?}", answer1.len()), format!("{}", part2(answer1)))
}

fn part1(input_lines: &[Vec<String>]) -> Vec<char> {
    let polymer = &input_lines[0][0];

    let mut chars = polymer.chars();
    let mut reduced_polymer = Vec::new();
    loop {
        let b = chars.next();
        match b {
            Some(b) => {
                if reduced_polymer.is_empty() {
                    reduced_polymer.push(b);
                }
                else if (reduced_polymer.last().unwrap().is_lowercase() != b.is_lowercase())  && reduced_polymer.last().unwrap().eq_ignore_ascii_case(&b) {
                    reduced_polymer.pop();
                }
                else {
                    reduced_polymer.push(b);
                }
            }
            None => {
                return reduced_polymer
            }
        }  
    }   
}

fn part2( polymer: Vec<char>) -> usize {
    let mut highest_length = polymer.len();
    for i in ASCII_LOWER {
        let mut polymer_clone = polymer.clone();
        polymer_clone.retain(|&x| !x.eq_ignore_ascii_case(&i));
        let new_length = part1(&[vec!(polymer_clone.iter().collect::<String>())]).len();
        if new_length < highest_length {
            highest_length = new_length
        }
    }
    highest_length
}
