use std::collections::HashMap;

use id_tree::*;

pub fn day08(input_lines: &[Vec<String>]) -> (String, String) {
    nice_input(input_lines);
    let answer1 = 0;
    let answer2 = 0;
    (format!("{:#?}", answer1), format!("{}", answer2))
}

#[derive(Debug)]
struct Level {
    height: i32,
    babies_completed_from_left: i32,
    nodes_on_level: Vec<Node>
}

fn build_level(level: i32) -> Level {
    Level { height: level, babies_completed_from_left: 0, nodes_on_level: Vec::new() }
}

#[derive(Debug)]
struct Node {
    babies_count: i32,
    metas_count: i32,
    metas: Vec<i32>
}

fn build_node(babies_count: i32, metas_count: i32) -> Node {
    Node{
        babies_count,
        metas_count,
        metas: Vec::new()
    }

}

fn nice_input(input_lines: &[Vec<String>]) {
    let convert: Vec<i32> = input_lines[0][0].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut list = convert.iter();
    let mut levels_of_nodes: HashMap<i32, Level> = HashMap::new();
    let mut level = 0;
    'outer: loop {  
        let b = list.next();
        println!("what level are we at now: {:#?}", level);
        match b {
            Some(b) => {
                println!("here's b: {:#?}", b);
                
                let metadata_quantity = list.next();
                match metadata_quantity {
                    Some(t) => {
                        println!("here;s t: {}", t);
                        levels_of_nodes.entry(level).or_insert(build_level(level)).nodes_on_level.push(build_node(b.clone(), t.clone()));
                        if b == &0 {
                            for j in 0..*t {
                                let p = list.next();
                                match p {
                                    Some(p) => {
                                        println!("Woooh we made it, here's t: {}", t);
                                        let baby_level = levels_of_nodes.entry(level).or_insert(build_level(level)).babies_completed_from_left;
                                        levels_of_nodes.entry(level).or_insert(build_level(level)).nodes_on_level[baby_level as usize].metas.push(p.clone());
                                    }
                                    None => {
                                        println!("oh dear these numbers aint matching")
                                    }
                                }
                            }
                            levels_of_nodes.entry(level).or_insert(build_level(level)).babies_completed_from_left += 1;
                            
                            'inner: loop {
                            if level == 0 {
                                break 'outer
                            }
                            let baby_level = levels_of_nodes.entry(level).or_insert(build_level(level)).babies_completed_from_left;
                            let baby_count = levels_of_nodes.entry(level-1).or_insert(build_level(level-1)).nodes_on_level.last().unwrap().babies_count;
                            //Check if all babies at that level consumed
                            println!("baby level: {:#?}", baby_level);
                            println!("baby count: {:#?}", baby_count);
                                if baby_level == baby_count {
                                    
                                    // go up a level
                                    level -= 1;
                                    let metadata_count = levels_of_nodes.entry(level).or_insert(build_level(level)).nodes_on_level.last().unwrap().metas_count;
                                    if metadata_count ==0 {
                                        levels_of_nodes.entry(level).or_insert(build_level(level)).babies_completed_from_left += 1;
                                        continue
                                    }
                                    else {
                                        for j in 0..metadata_count {
                                            println!("META BABBBBBBY: {}", j);
                                            let p = list.next();
                                            match p {
                                                Some(p) => {
                                                    println!("Woooh we made it in inner, here's t: {}", t);
                                                    let baby_level = levels_of_nodes.entry(level).or_insert(build_level(level)).babies_completed_from_left;
                                                    println!("here's the meta we gone push: {}", p.clone());
                                                    levels_of_nodes.entry(level).or_insert(build_level(level)).nodes_on_level[baby_level as usize -1 ].metas.push(p.clone());
                                                }
                                                None => {
                                                    println!("oh dear these numbers aint matching")
                                                }
                                            }
                                        }
                                    }  
                                }
                                else {
                                    println!("breaking cuz we got more babies to handle");
                                    break 'inner;
                                }
                            }
            
                        }
                        else {
                            println!("wooh lets increment babies completed on level {}", level);
                            levels_of_nodes.entry(level).or_insert(build_level(level)).babies_completed_from_left += 1;
                            level +=1;
                        }
                        
                    }
                    None => {
                        println!("here's the good stuff {:#?}", levels_of_nodes);
                        // println!("here's the good stuff {:#?}", levels_of_nodes.get(&0).unwrap());
                        return
                    }
                }
                                
            }
            None => {
                println!("here's the good stuff {:#?}", levels_of_nodes);
                return
            }
        } 
    } 
    println!("here's the good stuff {:#?}", levels_of_nodes);
    
}


#[cfg(test)]
mod tests {
    use super::day08;
    use crate::utils::load_input;

    #[test]
    fn check_day08_case08() {
        full_test(
"", // INPUT STRING
"0", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day08(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}
