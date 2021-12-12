use std::{fs, collections::HashSet};

fn neighbouring_coordinates(board: &Vec<Vec<usize>>, x: usize, y: usize) -> HashSet<(usize, usize)> {

    let x_max = board.len();
    let y_max = board[0].len();
    
    let mut neighbours: HashSet<(usize, usize)> = HashSet::new();

    for rx in ((x as i32 - 1).max(0))..((x + 2).min(x_max) as i32) {
        for ry in ((y as i32 - 1).max(0))..((y + 2).min(y_max) as i32) {

            if (rx as usize == x) & (ry as usize == y) {
                continue;
            }

            neighbours.insert((rx as usize, ry as usize));
        }
    }

    return neighbours;
}

fn propagate_flashes(board: &mut Vec<Vec<usize>>, flashed: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {

    let mut flash_copy = flashed.clone();

    for x in 0..board.len() {
        for y in 0..board[0].len() {

            let coords = (x, y);

            if flash_copy.contains(&coords) {
                continue;
            }

            // New flash!
            if board[x][y] > 9 {
                flash_copy.insert(coords);

                for (nx, ny) in neighbouring_coordinates(&board, x, y) {
                    board[nx][ny] += 1;
                    if board[nx][ny] > 9 {
                        flash_copy.extend(propagate_flashes(board, &flash_copy));
                    }
                }
            }
        }
    }

    return flash_copy;
}

fn main () {
    let file_content = fs::read_to_string("src\\day_11_input.txt").expect("File reading error");
    
    let mut board = file_content.split("\r\n")
        .map(|r| r.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()
    ).collect::<Vec<Vec<usize>>>();

    let mut flashes = 0 as usize;
    let simualtion_steps = 100;

    for _ in 0..simualtion_steps {

        for x in 0..board.len() {
            for y in 0..board[0].len() {
                board[x][y] += 1;
            }
        }

        let empty_flashes: HashSet<(usize, usize)> = HashSet::new();
        let flashed_tiles = propagate_flashes(&mut board, &empty_flashes);
        flashes += flashed_tiles.len();
        
        // Flashed tiles to 0
        for (x, y) in flashed_tiles {
            board[x][y] = 0;
        }
    }


    println!("{:?}", board);
    println!("{:?}", flashes);

}

#[test]
fn returns_neighbouring_coordinates() {
    let board = vec![vec![0,0,0], vec![0,0,0], vec![0,0,0]];

    let mut expected: HashSet<(usize, usize)> = HashSet::new();
    expected.extend([(0,0), (0,1), (0,2), (1,0), (1,2), (2,0), (2,1), (2,2)].iter());

    assert_eq!(neighbouring_coordinates(&board, 1, 1), expected);
}

#[test]
fn confers_to_given_example() {
    let mut board = vec![
        vec![5,4,8,3,1,4,3,2,2,3],
        vec![2,7,4,5,8,5,4,7,1,1],
        vec![5,2,6,4,5,5,6,1,7,3],
        vec![6,1,4,1,3,3,6,1,4,6],
        vec![6,3,5,7,3,8,5,4,7,8],
        vec![4,1,6,7,5,2,4,6,4,5],
        vec![2,1,7,6,8,4,1,7,2,1],
        vec![6,8,8,2,8,8,1,1,3,4],
        vec![4,8,4,6,8,4,8,5,5,4],
        vec![5,2,8,3,7,5,1,5,2,6]
        ];

    let mut flashes = 0 as usize;
    let simualtion_steps = 100;

    for _ in 0..simualtion_steps {

        for x in 0..board.len() {
            for y in 0..board[0].len() {
                board[x][y] += 1;
            }
        }

        let empty_flashes: HashSet<(usize, usize)> = HashSet::new();
        let flashed_tiles = propagate_flashes(&mut board, &empty_flashes);
        flashes += flashed_tiles.len();
        
        for (x, y) in flashed_tiles {
            board[x][y] = 0;
        }
    }

    assert_eq!(flashes, 1656);
}

