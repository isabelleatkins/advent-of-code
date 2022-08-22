use id_tree::*;

pub fn day09(input_lines: &[Vec<String>]) -> (String, String) {
    nice_input(input_lines);
    let answer1 = 0;
    let answer2 = 0;
    (format!("{:#?}", answer1), format!("{}", answer2))
}

struct Board {
    centre_marble: (i32, i32),
    order: Vec<i32>
}

impl Board {
    fn new() -> Board {
        Board {
            centre_marble: (0, 0),
            order: vec!(0)
        }
    }
}

fn nice_input(input_lines: &[Vec<String>]) {
    let marble_count = 7105800;
    let player_count: i32 = 491;
    let mut scores_on_the_doors: Vec<i32> = vec![0; player_count as usize];
    let mut board: Board = Board::new();
    let mut turn = 1;
    for marble in 1..marble_count {
        if marble%23 ==0 {
            scores_on_the_doors[(turn%player_count) as usize] += marble;
            if board.centre_marble.0 >= 7 {
                let index = board.centre_marble.0 -7;
                let number = board.order[index as usize];
                scores_on_the_doors[(turn%player_count) as usize] += number;
                board.order.remove(index as usize);
                board.centre_marble = (index, number);
            } else {
                let index = board.order.len() as i32  + (board.centre_marble.0-7);
                let number = board.order[index as usize];
                scores_on_the_doors[(turn%player_count) as usize] += number;
                board.order.remove(index as usize);
                board.centre_marble = (index, number);
            }
        }
        else if board.centre_marble.0 as usize + 1 == board.order.len() {
            board.order.insert(1, marble);
            board.centre_marble = (1, marble)
        }
        else if board.centre_marble.0 as usize == board.order.len() {
            board.order.push(marble);
            board.centre_marble = (board.order.len() as i32 - 1, marble)
        }
        else {
            board.order.insert(board.centre_marble.0 as usize + 2, marble);
            board.centre_marble = (board.centre_marble.0 + 2, marble)
        }
        turn += 1;  
    }
    println!("here's the max score: {:#?}", scores_on_the_doors.iter().max())
}
