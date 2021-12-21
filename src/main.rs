use std::collections::HashSet;
use std::{fs};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
struct Coords {
    x: usize,
    y: usize,
}

fn fold_x (col: usize, dots: Vec<Coords>) -> Vec<Coords> {

    let max_x = dots.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;

    // Left side larger
    if (max_x - col) <= col {
        return dots.iter().clone().map(|dot| {
            if dot.x > col {
                return Coords {
                    x: col - (dot.x - col),
                    y: dot.y,
                }
            } else {
                return Coords {
                    x: dot.x,
                    y: dot.y,
                }
            }
        }).collect::<Vec<Coords>>();
    }

    // Right side larger
    return dots.iter().clone().map(|dot| {
        if dot.x > col {
            return Coords {
                x: max_x - dot.x,
                y: dot.y,
            }
        } else {
            return Coords {
                x: (max_x - col) - (col - dot.x),
                y: dot.y,
            }
        }
    }).collect::<Vec<Coords>>();    
}

#[test]
fn left_dominant_odd_col_folds_correctly() {
    let dot = Coords {
        x: 4,
        y: 0
    };

    assert_eq!(fold_x(3, vec![dot]), vec![Coords{x: 2, y: 0}]);
}

#[test]
fn right_dominant_odd_col_folds_correctly() {
    let dot = Coords {
        x: 5,
        y: 0
    };
    let dot2 = Coords {
        x: 0,
        y: 0,
    };

    assert_eq!(fold_x(2, vec![dot, dot2]), vec![Coords{x: 0, y: 0}, Coords{x: 1, y: 0}]);
}



fn main () {
    let file_content = fs::read_to_string("src\\day_13_input.txt").expect("Something went wrong reading the file");

    let dots_and_folds = file_content.split("\r\n\r\n").collect::<Vec<&str>>();
    let (raw_dots, raw_folds) = (dots_and_folds[0], dots_and_folds[1]);
    
    let dots = raw_dots.split("\r\n")
        .map(|row| {
            let str_coords = row.split(",").collect::<Vec<&str>>();
            return Coords {
                x: usize::from_str_radix(str_coords[0], 10).unwrap(),
                y: usize::from_str_radix(str_coords[1], 10).unwrap(),
            };
        }).collect::<Vec<Coords>>();
    
    // let max_coords = dots.iter().cloned().reduce(|max, cs| {
    //     return Coords {
    //         x: if max.x > cs.x { max.x } else { cs.x },
    //         y: if max.y > cs.y { max.y } else { cs.y }
    //     }
    // }).unwrap();

    // let mut paper = vec![vec![false; max_coords.y + 1]; max_coords.x + 1];

    // dots.iter().for_each(|coords| paper[coords.x][coords.y] = true);

    // println!("{:?}", paper);

    let folds = raw_folds.split("\r\n")
        .map(|fold| {
            let mut parts = fold.split("=");
            return (parts.next().unwrap().chars().last().unwrap(), usize::from_str_radix(parts.next().unwrap(), 10).unwrap())
        }).collect::<Vec<(char, usize)>>();

    let remaining_dots: HashSet<Coords> = HashSet::from_iter(fold_x(folds[0].1, dots.clone()).iter().cloned());
    let orig_dots: HashSet<Coords> = HashSet::from_iter(dots.iter().cloned());

    println!("{:?}", remaining_dots.len());
}