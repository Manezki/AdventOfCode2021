use std::fs;

// Seems to work correctly :+1:
fn check_mask_for_win(mask: &Vec<Vec<i32>>) -> bool {

    let mut col_sums = vec![0; mask.len()];
    let mut row_sums = vec![0; mask.len()];

    for row in 0..mask.len() {
        for col in 0..mask[row].len() {
            let value = mask[row][col];
            if value == 1 {
                col_sums[col] += 1;
                row_sums[row] += 1;
            }
        }
    }

    return col_sums.contains(&mask.len()) || row_sums.contains(&mask.len())
}

fn main() {
    let file_content = fs::read_to_string("C:\\Users\\mande\\Projects\\Advent-Of-Code\\src\\day_4_input.txt")
        .expect("Something went horribly wrong!");

    let called_numbers = &file_content.split("\r\n").collect::<Vec<&str>>()[0].split(",").into_iter().map(|c| i32::from_str_radix(&c, 10).unwrap()).collect::<Vec<i32>>();

    let board_inputs = &file_content.split("\r\n\r\n").collect::<Vec<&str>>()[1..];
    let boards = board_inputs.iter()
        .map(|b| b.split("\r\n")
            .into_iter().map(|r| r.split(" ").into_iter()
                .filter_map(|n| i32::from_str_radix(n, 10).ok()).collect::<Vec<i32>>()
            ).collect::<Vec<Vec<i32>>>()
        ).collect::<Vec<Vec<Vec<i32>>>>();

    let mut board_masks = vec![vec![vec![0; 5]; 5]; boards.len()];

    let mut won_boards = vec![false; boards.len()];
    let mut num_won_boards = 0;

    let mut losing_board_idx = 0;
    let mut final_call = -1;
    let mut finished = false;

    while !finished {
        for call_number in called_numbers {
            for (board_number, (board, mask)) in boards.iter().zip(board_masks.iter_mut()).enumerate() {
                for row in 0..board.len() {
                    for col in 0..board[row].len() {
                        
                        let value = board[row][col];
                        if value == *call_number {
                            mask[row][col] = 1;
                        }

                        if check_mask_for_win(mask) {

                            if !won_boards[board_number] {
                                num_won_boards += 1;
                                won_boards[board_number] = true;
                            }

                            // Losing board is hitting a Bingo
                            if num_won_boards == 100 {
                                losing_board_idx = board_number;
                                final_call = *call_number;
                                finished = true;
                            }
                        }
                        
                        if finished {
                            break;
                        }
                    }
                    
                    if finished {
                        break;
                    }
                }

                if finished {
                    break;
                }
            }

            if finished {
                break;
            }
        }
    }

    let unmarked_sum = boards[losing_board_idx].iter().zip(board_masks[losing_board_idx].iter())
        .flat_map(|(br, mr)| br.iter().zip(mr.iter())
            .map(|(bc, mc)| return *bc * ((*mc * -1)+1))
        ).sum::<i32>();

    println!("{}", unmarked_sum);
    println!("{}", final_call);
    println!("{}", final_call*unmarked_sum);

}
