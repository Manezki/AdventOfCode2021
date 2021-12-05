use std::fs;
use std::collections::HashMap;

fn vector_points(start: &(i32, i32), end: &(i32, i32)) -> Vec<(i32, i32)>
    // where T: Eq + PartialOrd + Copy + std::fmt::Debug + std::ops::Sub<T, Output= T> + std::ops::Mul<T, Output = T>,
    // std::ops::Range<T>: Iterator<Item = T>,
    // std::ops::RangeInclusive<T>: Iterator<Item = T>,
    // std::ops::Sub<T>: T,
    // dyn Output<T as Sub>: Abs
    {

    let mut res: Vec<(i32, i32)> = Vec::new();

    // Vertical
    if start.0 == end.0 {

        let y_range = if start.1 > end.1 { end.1..=start.1 } else { start.1..=end.1 };

        for y in y_range {
            res.push((start.0, y));
        }
    // Horizontal
    } else if start.1 == end.1 {

        let x_range = if start.0 > end.0 { end.0..=start.0 } else { start.0..=end.0 };

        for x in x_range {
            res.push((x, start.1));
        }
    // Diagonal
    } else if ((start.0 - end.0) * (start.0 - end.0)) == ((start.1 - end.1) * (start.1 - end.1)) {
        
        let angle: i32 = if (start.1 - end.1) > 0 { -1 } else { 1 };

        if start.0 > end.0 {
            for (i, x) in (end.0..=start.0).enumerate() {
                res.push((start.0 - (i as i32), start.1 + angle*(i as i32)));
            }
        } else {
            for (i, x) in (start.0..=end.0).enumerate() {
                res.push((x, start.1 + angle*(i as i32)));
            }
        }


        // let x_range = if start.0 > end.0 { (start.0..=end.0).rev() } else { start.0..=end.0 };
    } else {
        panic!("Linetype was not diagonal, horizontal, nor vertical.")
    }

    return res;
}

fn main() {
    let day_input = fs::read_to_string("src\\day_5_input.txt")
        .expect("Something went horribly wrong reading the file");
    let vector_tuples = day_input.split("\r\n")
        .map(|str_vec| {
            let line_parts = str_vec.split("->")
                .map(|point| {
                    let int_dims = point.split(",")
                        .map(|dim| {
                            i32::from_str_radix(dim.trim(), 10).unwrap()
                        }).collect::<Vec<i32>>();
                    return (int_dims[0], int_dims[1]);
                    }
                ).collect::<Vec<(i32, i32)>>();
            return (line_parts[0], line_parts[1])
        }).collect::<Vec<((i32, i32), (i32, i32))>>();

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for vector in vector_tuples {
        for point in vector_points(&vector.0, &vector.1) {
            let count = *map.entry(point).or_insert(0);
            map.insert(point, count + 1);
        }
    }

    println!("{:?}", map.values().map(|count| if *count > 1 { 1 } else { 0 }).sum::<i32>());
}

#[test]
fn diagonal_up_right() {
    assert_eq!(vector_points(&(1, 3), &(3, 5)), vec![(1, 3), (2, 4), (3, 5)])
}

#[test]
fn diagonal_down_left() {
    assert_eq!(vector_points(&(3, 5), &(1, 3)), vec![(3, 5), (2, 4), (1, 3)])
}

#[test]
fn diagonal_up_left() {
    assert_eq!(vector_points(&(3, 3), &(1, 5)), vec![(3, 3), (2, 4), (1, 5)])
}

#[test]
fn diagonal_down_right() {
    assert_eq!(vector_points(&(1, 5), &(3, 3)), vec![(1, 5), (2, 4), (3, 3)])
}
