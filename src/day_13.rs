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

fn fold_y (row: usize, dots: Vec<Coords>) -> Vec<Coords> {

    let max_y = dots.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    // Top side larger
    if (max_y - row) <= row {
        return dots.iter().clone().map(|dot| {
            if dot.y > row {
                return Coords {
                    x: dot.x,
                    y: row - (dot.y - row),
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
        if dot.y > row {
            return Coords {
                x: dot.x,
                y: max_y - dot.y,
            }
        } else {
            return Coords {
                x: dot.x,
                y: (max_y - row) - (row - dot.y),
            }
        }
    }).collect::<Vec<Coords>>();    
}


#[test]
fn top_dominant_odd_row_folds_correctly() {
    let dot = Coords {
        x: 0,
        y: 4
    };

    assert_eq!(fold_y(3, vec![dot]), vec![Coords{x: 0, y: 2}]);
}

#[test]
fn bottom_dominant_odd_row_folds_correctly() {
    let dot = Coords {
        x: 0,
        y: 5
    };
    let dot2 = Coords {
        x: 0,
        y: 0,
    };

    assert_eq!(fold_y(2, vec![dot, dot2]), vec![Coords{x: 0, y: 0}, Coords{x: 0, y: 1}]);
}



fn main () {
    let file_content = fs::read_to_string("src\\day_13_input.txt").expect("Something went wrong reading the file");

    let dots_and_folds = file_content.split("\r\n\r\n").collect::<Vec<&str>>();
    let (raw_dots, raw_folds) = (dots_and_folds[0], dots_and_folds[1]);
    
    let mut dots = raw_dots.split("\r\n")
        .map(|row| {
            let str_coords = row.split(",").collect::<Vec<&str>>();
            return Coords {
                x: usize::from_str_radix(str_coords[0], 10).unwrap(),
                y: usize::from_str_radix(str_coords[1], 10).unwrap(),
            };
        }).collect::<Vec<Coords>>();

    let folds = raw_folds.split("\r\n")
        .map(|fold| {
            let mut parts = fold.split("=");
            return (parts.next().unwrap().chars().last().unwrap(), usize::from_str_radix(parts.next().unwrap(), 10).unwrap())
        }).collect::<Vec<(char, usize)>>();

    for (dim, idx) in folds {
        match dim {
            'x' => dots = fold_x(idx, dots),
            'y' => dots = fold_y(idx, dots),
            _ => panic!("Unrecognized dimension")
        }
    }

    let max_coords = dots.iter().cloned().reduce(|c, d| {
        return Coords {
            x: if c.x < d.x { d.x } else { c.x },
            y: if c.y < d.y { d.y } else { c.y },
        };
    }).unwrap();

    // Flipped x&y indices - helps printing later
    let mut paper = vec![vec!["."; max_coords.x + 1]; max_coords.y + 1];
    dots.iter().for_each(|dot| paper[dot.y][dot.x] = "x");

    paper.iter().for_each(|row| println!("{:?}", row.join("")));
}