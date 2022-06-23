use id_tree::*;

pub fn day08(input_lines: &[Vec<String>]) -> (String, String) {
    nice_input(input_lines);
    let answer1 = 0;
    let answer2 = 0;
    (format!("{:#?}", answer1), format!("{}", answer2))
}

fn nice_input(input_lines: &[Vec<String>]) {
    let convert: Vec<i64> = input_lines[0][0].split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

    let mut list = convert.iter();
    let mut all_metadatas: Vec<&i64> = Vec::new();
    loop {
        let b = list.next();
        match b {
            Some(b) => {
                // println!("here's b: {:#?}", b);
                let metadata_quantity = list.next();
                match metadata_quantity {
                    Some(t) => {
                        all_metadatas.push(t);
                    }
                    None => {
                        println!("here's the good stuff {:#?}", all_metadatas);
                        return
                    }
                }
                
            }
            None => {
                println!("here's the good stuff {:#?}", all_metadatas[0]);
                return
            }
        } 
    } 
    println!("here's the good stuff {:#?}", all_metadatas);
    
}



fn make_tree() {
    use id_tree::InsertBehavior::*;

    //      0
    //     / \
    //    1   2
    //   / \
    //  3   4
    let mut tree: Tree<i32> = TreeBuilder::new()
        .with_node_capacity(5)
        .build();

    let root_id: NodeId = tree.insert(Node::new(0), AsRoot).unwrap();
    let child_id: NodeId = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(3), UnderNode(&child_id)).unwrap();
    tree.insert(Node::new(4), UnderNode(&child_id)).unwrap();

    println!("Pre-order:");
    for node in tree.traverse_pre_order(&root_id).unwrap() {
        print!("{}, ", node.data());
    }
    // results in the output "0, 1, 3, 4, 2, "
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
