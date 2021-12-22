use std::collections::binary_heap;
use std::fmt::Binary;
use std::{fs, collections::HashSet, collections::BinaryHeap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
struct Edge {
    location: (usize, usize),
    risk: u32
}


// Shameless borrow from https://doc.rust-lang.org/std/collections/binary_heap/index.html
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.risk.cmp(&self.risk)
            .then_with(|| self.location.cmp(&other.location))
    }
}

fn main_axis_neighbours (x: usize, y: usize, max_coords: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut res: HashSet<(usize, usize)> = HashSet::new();

    let candidates = vec![
        (x as i32 - 1, y as i32), (x  as i32 + 1, y as i32), (x as i32, y  as i32 - 1), (x as i32, y as i32 + 1)
    ];

    for (dx, dy) in candidates {
        if (dx < 0) | (dx > max_coords.0 as i32) {
            continue;
        } else if (dy < 0) | (dy > max_coords.1 as i32) {
            continue;
        }

        res.insert((dx as usize, dy as usize));
    }

    // println!("{:?} - {:?}", (x, y), res);
    assert!(res.len() <= 4, "Node can have at most 4 neighbours");

    return res;
}

// Too slow
fn naive_find_path (board: &Vec<Vec<u32>>, visited: &HashSet<(usize, usize)>, location: (usize, usize)) -> (Vec<(usize, usize)>, u32) {

    let max_coords = (board.len()-1, board[0].len()-1);
    let mut expanded_visited = visited.clone();
    expanded_visited.insert(location);

    let risk = board[location.0][location.1];

    if max_coords == location {
        return (vec![location], risk);
    }

    let mut min_risk = 999 as u32;
    let mut min_risk_path: Vec<(usize, usize)> = Vec::new();

    for neighbour in main_axis_neighbours(location.0, location.1, max_coords) {
        if visited.contains(&neighbour) {
            continue;
        }
        let (path, risk) = naive_find_path(board, &expanded_visited, neighbour);

        if risk < min_risk {
            min_risk = risk;
            min_risk_path = path;
        }
    }

    min_risk_path.insert(0, location);
    let total_risk = if location == (0, 0) { min_risk } else { min_risk + risk };

    return (min_risk_path, total_risk);
}

// Too complex
fn naive_inverted_path (
        board: &Vec<Vec<u32>>,
        visited: &HashSet<(usize, usize)>,
        location: (usize, usize),
        acc_risk: u32,
        acc_path: &Vec<(usize, usize)>,
        global_best_risk: u32,
        global_best_path: &Vec<(usize, usize)>
    ) -> (Vec<(usize, usize)>, u32) {

    let risk = board[location.0][location.1];
    let accumulated_risk = acc_risk + risk;

    // println!("{:?}", location);

    if (0, 0) == location {
        return (acc_path.clone(), acc_risk);
    } else if accumulated_risk >= global_best_risk {
        return (global_best_path.clone(), global_best_risk);
    }

    println!("{:?} < {:?}", accumulated_risk, global_best_risk);

    let max_coords = (board.len()-1, board[0].len()-1);
    let mut expanded_visited = visited.clone();
    expanded_visited.insert(location);

    let mut cumulative_path = acc_path.clone();
    cumulative_path.insert(0, location);

    let mut local_best_risk = global_best_risk;
    let mut local_best_path = global_best_path.clone();

    for neighbour in main_axis_neighbours(location.0, location.1, max_coords) {
        if visited.contains(&neighbour) {
            continue;
        }
        // println!("Recursing to {:?}", neighbour);
        let (path, risk) = naive_inverted_path(board, &expanded_visited, neighbour, accumulated_risk, &cumulative_path, local_best_risk, &global_best_path);

        if risk < local_best_risk {
            local_best_risk = risk;
            local_best_path = path.clone();
        }
    }

    return (local_best_path.clone(), local_best_risk);
}
fn dijkstra_risk (board: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    let max_coords = (board.len() - 1, board[0].len() - 1);

    let mut cumulative_risk = vec![vec![u32::MAX - 10; board.len()]; board[0].len()];
    cumulative_risk[0][0] = 0;

    let mut min_heap: BinaryHeap<Edge> = BinaryHeap::new();
    min_heap.push(Edge { location: (0, 0), risk: 0 });

    while let Some(Edge { location, risk }) = min_heap.pop() {
        
        if risk > cumulative_risk[location.0][location.1] {
            continue;
        }

        for neighbour in main_axis_neighbours(location.0, location.1, max_coords) {
            let estimated_risk = cumulative_risk[neighbour.0][neighbour.1];
            if (board[neighbour.0][neighbour.1] + risk) < estimated_risk {
                let step = Edge { location: neighbour, risk: (board[neighbour.0][neighbour.1] + risk) };
                min_heap.push(step);
                cumulative_risk[neighbour.0][neighbour.1] = board[neighbour.0][neighbour.1] + risk;
            }
        }
    }

    return cumulative_risk;
}

#[test]
fn risk_confers_to_example() {
    let example = vec![
        vec![1,1,6,3,7,5,1,7,4,2],
        vec![1,3,8,1,3,7,3,6,7,2],
        vec![2,1,3,6,5,1,1,3,2,8],
        vec![3,6,9,4,9,3,1,5,6,9],
        vec![7,4,6,3,4,1,7,1,1,1],
        vec![1,3,1,9,1,2,8,1,3,7],
        vec![1,3,5,9,9,1,2,4,2,1],
        vec![3,1,2,5,4,2,1,6,3,9],
        vec![1,2,9,3,1,3,8,5,2,1],
        vec![2,3,1,1,9,4,4,5,8,1]
    ];

    let risks = dijkstra_risk(&example);
    risks.iter().for_each(|r| println!("{:?}", r));

    assert_eq!(risks[risks.len() - 1][risks[0].len() - 1], 40);
}

fn main () {
    let file_content = fs::read_to_string("src\\day_15_input.txt").expect("Something went wrong reading the file");
    
    let board = file_content.split("\r\n")
        .map(|row| row.chars()
            .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap()).collect::<Vec<u32>>()
        ).collect::<Vec<Vec<u32>>>();

    // let paths = dijkstra_risk(&board);

    // println!("Least risky path puzzle 1: {:?}", paths[paths.len() - 1][paths[0].len() - 1]);

    let mut extended_board: Vec<Vec<u32>> = vec![vec![0; board.len()*5]; board[0].len()*5];
    
    for x in 0..5 {
        for y in 0..5 {
            let addition = (x + y) as u32;

            for i in 0..board.len() {
                for j in 0..board[0].len() {
                    let old_value = board[i][j];
                    extended_board[x*board.len() + i][y*board[0].len() + j] = if old_value + addition <= 9 { old_value + addition } else { ((old_value + addition)%10) + 1 };
                }
            }
        }
    }

    // Manageable with a cargo build --release : )
    let paths = dijkstra_risk(&extended_board);

    println!("Least risky path puzzle 2: {:?}", paths[paths.len() - 1][paths[0].len() - 1]);
}