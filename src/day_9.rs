use std::fs;
use std::collections::HashSet;

// , ignore_x: usize, ignore_y: usize
fn is_basin(input: &Vec<Vec<u32>>, x: usize, y: usize, built_basin: &HashSet<(usize, usize)>) -> bool {

    let max_x = input.len();
    let max_y = input[0].len();

    let mut basin = true;

    if input[x][y] == 9 {
        return false;
    }

    for rx in (x as i32) - 1..=(x as i32) + 1 {
        if (rx < 0) | (rx >= max_x as i32 - 1) {
            continue;
        }
        for ry in (y as i32) - 1..=(y as i32) + 1 {
            if (ry < 0) | (ry >= max_y as i32 - 1) {
                continue;
            }
            
            if (rx == x as i32) & (ry == y as i32) {
                continue;
            } else if built_basin.contains(&(rx as usize, ry as usize)) {
                // println!("Contains {:?}, - {:?}, {:?}", built_basin, rx, ry);
                continue
            } else if (rx == x as i32) | (ry == y as i32) {
                if input[rx as usize][ry as usize] < input[x][y] {
                    basin = false;
                }
            }
        }
    }

    return basin;
}

fn convolve_low_point(input: &Vec<Vec<u32>>, x: usize, y: usize) -> Option<u32> {
    // Returns the RISK if given point is low-point
    
    let max_x = input.len();
    let max_y = input[0].len();

    let mut low_point = true;

    for rx in (x as i32) - 1..=(x as i32) + 1 {
        if (rx < 0) | (rx > max_x as i32 - 1) {
            continue;
        }
        for ry in (y as i32) - 1..=(y as i32) + 1 {
            if (ry < 0) | (ry > max_y as i32 - 1) {
                continue;
            }
            if (rx == x as i32) & (ry == y as i32) {
                continue;
            } else if (rx == x as i32) | (ry == y as i32) {
                if input[rx as usize][ry as usize] <= input[x][y] {
                    low_point = false;
                }
            }
        }
    }

    if low_point {
        return Some(input[x][y] + 1);
    } else {
        return None
    }
}


fn traverse_basin(input: &Vec<Vec<u32>>, x: usize, y: usize, basin: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {

    let mut resulting_basin: HashSet<(usize, usize)> = HashSet::new();
    resulting_basin.extend(basin);

    // Check self
    if !is_basin(&input, x, y, &resulting_basin) {
        // println!("From {:?}, Reject {:?}, {:?}", basin, x, y);
        return resulting_basin;
    }
    // println!("From {:?}, Accept {:?}, {:?}", basin, x, y);

    resulting_basin.insert((x, y));

    let max_x = input.len();
    let max_y = input[0].len();

    for rx in (x as i32) - 1..=(x as i32) + 1 {
        if (rx < 0) | (rx > max_x as i32 - 1) {
            continue;
        }
        for ry in (y as i32) - 1..=(y as i32) + 1 {
            if (ry < 0) | (ry > max_y as i32 - 1) {
                continue;
            }

            if resulting_basin.contains(&(rx as usize, ry as usize)) {
                // println!("Contains {:?}, - {:?}, {:?}", resulting_basin, rx, ry);
                continue
            } else if (rx == x as i32) | (ry == y as i32) {
                // println!("Init recursion with {:?}, - {:?}, {:?}", resulting_basin, rx, ry);
                // traverse_basin(&input, rx as usize, ry as usize, &resulting_basin);
                resulting_basin.extend(&traverse_basin(&input, rx as usize, ry as usize, &resulting_basin));
            }
        }
    }

    return resulting_basin;
}

fn main() {
    let file_content = fs::read_to_string("src\\day_9_input.txt").expect("Reading the input failed");
    let file_lines = file_content.split("\r\n").map(
        |l| l.chars().into_iter().map(|c| u32::from_str_radix(&(c.to_string()), 10).unwrap()).collect::<Vec<u32>>()
    ).collect::<Vec<Vec<u32>>>();

    let mut largest_basins = vec![0; 3];

    for x in 0..file_lines.len() {
        for y in 0..file_lines[0].len() {
            match convolve_low_point(&file_lines, x, y) {
                Some(_) => {
                    let mut basin: HashSet<(usize, usize)> = HashSet::new();
                    basin.insert((x, y));
                    let size = traverse_basin(&file_lines, x, y, &basin).len();
                    
                    for i in 0..largest_basins.len() {
                        if largest_basins[i] < size {
                            largest_basins[i] = size;
                            break;
                        }
                    }
                },
                None => (),
            }
        }
    }

    println!("{:?}", largest_basins);
}


#[test]
fn identify_low_point() {

    let input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];

    assert_eq!(convolve_low_point(&input, 1, 1), Some(1));
}


#[test]
fn reject_high_point() {

    let input = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

    assert_eq!(convolve_low_point(&input, 1, 1), None);
}

#[test]
fn edge_low_point() {

    let input = vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

    assert_eq!(convolve_low_point(&input, 0, 0), Some(1));
}

#[test]
fn basin_ignores_parent_node() {

    let input = vec![vec![2, 0, 2], vec![2, 1, 2], vec![2, 2, 2]];
    let mut basin: HashSet<(usize, usize)> = HashSet::new();
    basin.insert((0, 1));

    assert_eq!(is_basin(&input, 1, 1, &basin), true);
}

#[test]
fn basin_traverses_to_neighbours() {

    let input = vec![vec![2, 0, 2], vec![2, 1, 2], vec![2, 2, 2]];
    let mut basin: HashSet<(usize, usize)> = HashSet::new();
    basin.insert((0, 1));

    assert_eq!(traverse_basin(&input, 0, 1, &basin).len(), 9);
}

// #[test]
// fn basin_rejects_to_neighbours() {

//     let input = vec![vec![2, 0, 2], vec![2, 1, 2], vec![2, 2, 2]];
//     let mut basin: HashSet<(usize, usize)> = HashSet::new();
//     basin.insert((0, 1));

//     assert_eq!(traverse_basin(&input, 0, 1, &basin).len(), 2);
// }

#[test]
fn given_example() {

    let input = vec![vec![2,1,9,9,9,4,3,2,1,0], vec![3,9,8,7,8,9,4,9,2,1], vec![9,8,5,6,7,8,9,8,9,2], vec![8,7,6,7,8,9,6,7,8,9], vec![9,8,9,9,9,6,5,6,7,8]];
    let mut basin: HashSet<(usize, usize)> = HashSet::new();
    basin.insert((2, 2));

    assert_eq!(traverse_basin(&input, 2, 2, &basin).len(), 14);
}
