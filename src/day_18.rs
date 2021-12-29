use std::fs;

#[derive(Clone)]
struct SnailfishNumber {
    left: Option<Box<SnailfishNumber>>,
    right: Option<Box<SnailfishNumber>>,
    data: (u32, u32)
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let left = if self.left.is_none() {self.data.0.to_string()} else {self.left.as_ref().unwrap().to_string()};
        let right = if self.right.is_none() {self.data.1.to_string()} else {self.right.as_ref().unwrap().to_string()};
        
        return write!(f, "[{},{}]", left, right)
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut res = Self {
            left: Some(Box::new(self)),
            right: Some(Box::new(rhs)),
            data: (0, 0)
        };
        res.reduce();
        return res;
    }
}

impl SnailfishNumber {
    fn explode(&mut self, depth: u8) -> Option<(u32, u32)> {

        // TODO Perform a single 'explode' event during the traverse

        if depth == 4 {
            // If at depth 4, node has any children - they need to be exploded.

            if self.left.is_none() & self.right.is_none() {
                return None;
            }
            // ----------------------------

            if let Some(left) = self.left.take().as_deref() {
                let ll = left.data.0;
                let lr = left.data.1;
                
                if self.right.is_some() {
                    let mut right = self.right.take().unwrap();
                    right.left_add(lr);
                    self.right = Some(right.clone());
                } else {
                    self.data.1 = self.data.1 + lr;
                }

                self.left = None;
                return Some((ll, 0));
            }

            if let Some(right) = self.right.take().as_deref() {
                let rl = right.data.0;
                let rr = right.data.1;

                if self.left.is_some() {
                    let mut left = self.left.take().unwrap();
                    left.right_add(rl);
                    self.left = Some(left.clone());
                } else {
                    self.data.0 = self.data.0 + rl;
                }

                self.right = None;
                return Some((0, rr));
            }
        }

        if let Some(left) = self.left.take().as_deref_mut() {
            if let Some((of_l, of_r)) = left.explode(depth + 1) {
                self.left = Some(Box::new(left.clone()));
                self.add((0, of_r));
                self.left = Some(Box::new(left.clone()));

                return Some((of_l, 0));
            }
            self.left = Some(Box::new(left.clone()));
        }

        if let Some(right) = self.right.take().as_deref_mut() {
            if let Some((of_l, of_r)) = right.explode(depth + 1) {
                self.right = Some(Box::new(right.clone()));
                self.add((of_l, 0));
                self.right = Some(Box::new(right.clone()));

                return Some((0, of_r));
            }
            self.right = Some(Box::new(right.clone()));
        }

        return None;
    }

    fn left_add(&mut self, rhs: u32) -> () {
        // Adds RHS to left-most leaf
        if let Some(left) = self.left.take().as_deref_mut() {
            left.left_add(rhs);
            self.left = Some(Box::new(left.clone()));
        } else {
            self.data = (self.data.0 + rhs, self.data.1);
        }
    }

    fn right_add(&mut self, rhs: u32) -> () {
        // Adds RHS to right-most leaf
        if let Some(right) = self.right.take().as_deref_mut() {
            right.right_add(rhs);
            self.right = Some(Box::new(right.clone()));
        } else {
            self.data = (self.data.0, self.data.1  + rhs);
        }
    }

    fn add(&mut self, rhs: (u32, u32)) -> () {

        if let Some(left) = self.left.take().as_deref_mut() {
            left.right_add(rhs.0);
            self.left = Some(Box::new(left.clone()));
        } else {
            self.data = (self.data.0 + rhs.0, self.data.1);
        }

        if let Some(right) = self.right.take().as_deref_mut() {
            right.left_add(rhs.1);
            self.right = Some(Box::new(right.clone()));
        } else {
            self.data = (self.data.0, self.data.1  + rhs.1);
        }
    }
    fn split(&mut self) -> bool {

        if let Some(left) = self.left.take().as_deref_mut() {
            if left.split() {
                self.left = Some(Box::new(left.clone()));
                return true;
            }
            self.left = Some(Box::new(left.clone()));
        } else {
            if self.data.0 >= 10 {
                let ll = self.data.0 / 2;
                let lr = ((self.data.0 as f32)/2.0).ceil() as u32;

                self.left = Some(Box::new(SnailfishNumber {
                    left: None,
                    right: None,
                    data: (ll, lr)
                }));
                self.data = (0, self.data.1);
                return true;
            }
        }

        if let Some(right) = self.right.take().as_deref_mut() {
            if right.split() {
                self.right = Some(Box::new(right.clone()));
                return true;
            }
            self.right = Some(Box::new(right.clone()))
        } else {
            if self.data.1 >= 10 {
                let rl = self.data.1 / 2;
                let rr = ((self.data.1 as f32)/2.0).ceil() as u32;

                self.right = Some(Box::new(SnailfishNumber {
                    left: None,
                    right: None,
                    data: (rl, rr)
                }));
                self.data = (self.data.0, 0);
                return true;
            }
        }
        
        return false;
    }
    fn magnitude(&self) -> u32 {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => return 3*left.magnitude() + 2*right.magnitude(),
            (Some(left), None) => return 3*left.magnitude() + 2*self.data.1,
            (None, Some(right)) => return 3*self.data.0 + 2*right.magnitude(),
            (None, None) => return 3*self.data.0 + 2*self.data.1,
        }
    }
    fn reduce(&mut self) -> () {
        let mut done = false;

        while !done {
            if self.explode(1).is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            done = true;
        }
    }
    fn parse(serialized: &str) -> SnailfishNumber {
        return SnailfishNumber::parse_next_snailfish_number(serialized).0
    }
    fn parse_next_snailfish_number(serialized: &str) -> (SnailfishNumber, usize) {
        let mut bits_read = 1;
        assert_eq!(serialized[0..1], *"[");

        let (left_child, left_data, bits) = {
            if serialized[1..2] == *"[" {
                let (child, read) = SnailfishNumber::parse_next_snailfish_number(&serialized[1..]);
                (Some(Box::new(child)), 0, read)
            } else {
                let mut bits_for_number = 1;
                while &serialized[bits_read + bits_for_number..bits_read + bits_for_number + 1] != "," {
                    bits_for_number += 1;
                }
                (None, u32::from_str_radix(&serialized[bits_read..bits_read + bits_for_number], 10).unwrap(), bits_for_number)
            }
        };

        bits_read += bits;

        // Check that left element is followed by a comma, and skip the comma
        assert_eq!(serialized[bits_read..bits_read+1], *",");
        bits_read += 1;

        let (right_child, right_data, bits) = {
            if serialized[bits_read..bits_read+1] == *"[" {
                let (child, read) = SnailfishNumber::parse_next_snailfish_number(&serialized[bits_read..]);
                (Some(Box::new(child)), 0, read)
            } else {
                let mut bits_for_number = 1;
                while &serialized[bits_read + bits_for_number..bits_read + bits_for_number + 1] != "]" {
                    bits_for_number += 1;
                }
                (None, u32::from_str_radix(&serialized[bits_read..bits_read + bits_for_number], 10).unwrap(), bits_for_number)
            }
        };

        bits_read += bits;

        // Check that right element is followed by a ], and skip the ]
        assert_eq!(serialized[bits_read..bits_read+1], *"]");
        bits_read += 1;

        return (SnailfishNumber {
            left: left_child,
            right: right_child,
            data: (left_data, right_data)
        }, bits_read)
    }
}

fn max_magnitude_of_sum(numbers: Vec<SnailfishNumber>) -> u32 {
    let mut max_magnitude = 0;

    for lhs_idx in 0..numbers.len() {
        for rhs_idx in 0..numbers.len() {

            if lhs_idx == rhs_idx {
                continue;
            }

            let lhs = numbers[lhs_idx].clone();
            let rhs = numbers[rhs_idx].clone();

            let sum = lhs + rhs;
            let mag = sum.magnitude();
            if mag > max_magnitude {
                max_magnitude = mag;
            }
        }
    }

    return max_magnitude;
}

#[test]
fn displays_according_to_examples () {
    let test = SnailfishNumber {
        left: Some(Box::new(SnailfishNumber {
            left: None,
            right: None,
            data: (1, 2)
        })),
        right: None,
        data: (0, 3)
    };
    assert_eq!(test.to_string(), "[[1,2],3]");
}

#[test]
fn addition_according_to_examples () {
    let mut test = SnailfishNumber {
        left: Some(Box::new(SnailfishNumber {
            left: None,
            right: None,
            data: (1, 2)
        })),
        right: None,
        data: (0, 3)
    };
    test.add((2, 1));
    assert_eq!(test.to_string(), "[[1,4],4]");

    let mut test = SnailfishNumber {
        left: Some(Box::new(SnailfishNumber {
            left: None,
            right: None,
            data: (1, 2)
        })),
        right: Some(Box::new(SnailfishNumber {
            left: None,
            right: None,
            data: (1, 2)
        })),
        data: (0, 0)
    };
    test.add((2, 1));
    assert_eq!(test.to_string(), "[[1,4],[2,2]]");

    let mut test = SnailfishNumber {
        left: Some(Box::new(SnailfishNumber {
            left: Some(Box::new(SnailfishNumber {
                left: None,
                right: None,
                data: (1, 1)
            })),
            right: Some(Box::new(SnailfishNumber {
                left: None,
                right: None,
                data: (1, 1)
            })),
            data: (0, 0)
        })),
        right: Some(Box::new(SnailfishNumber {
            left: Some(Box::new(SnailfishNumber {
                left: None,
                right: None,
                data: (1, 1)
            })),
            right: Some(Box::new(SnailfishNumber {
                left: None,
                right: None,
                data: (1, 1)
            })),
            data: (1, 1)
        })),
        data: (0, 0)
    };
    test.add((2, 1));
    assert_eq!(test.to_string(), "[[[1,1],[1,3]],[[2,1],[1,1]]]");
}

#[test]
fn parses_according_to_examples () {
    let test = SnailfishNumber::parse("[[[[[9,8],1],2],3],4]");
    assert_eq!(test.to_string(), "[[[[[9,8],1],2],3],4]");
}

#[test]
fn explodes_according_to_examples () {
    let mut bt = SnailfishNumber::parse("[[[[[9,8],1],2],3],4]");
    bt.explode(1);
    assert_eq!(bt.to_string(), "[[[[0,9],2],3],4]");

    let mut bt = SnailfishNumber::parse("[7,[6,[5,[4,[3,2]]]]]");
    bt.explode(1);
    assert_eq!(bt.to_string(), "[7,[6,[5,[7,0]]]]");

    let mut bt = SnailfishNumber::parse("[[6,[5,[4,[3,2]]]],1]");
    bt.explode(1);
    assert_eq!(bt.to_string(), "[[6,[5,[7,0]]],3]");

    let mut bt = SnailfishNumber::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    bt.explode(1);
    assert_eq!(bt.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

    let mut bt = SnailfishNumber::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    bt.explode(1);
    assert_eq!(bt.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
}

#[test]
fn splits_according_to_examples () {
    let mut bt = SnailfishNumber::parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    bt.split();
    assert_eq!(bt.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");

    // Should ONLY split the number 15, as explode needs to be carried out before splits
    let mut bt = SnailfishNumber::parse("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    bt.split();
    assert_eq!(bt.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
}

#[test]
fn reduces_according_to_examples () {
    let mut bt = SnailfishNumber::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    bt.reduce();
    assert_eq!(bt.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn magnitude_according_to_examples () {
    assert_eq!(SnailfishNumber::parse("[[1,2],[[3,4],5]]").magnitude(), 143);
    assert_eq!(SnailfishNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
    assert_eq!(SnailfishNumber::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
    assert_eq!(SnailfishNumber::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
    assert_eq!(SnailfishNumber::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
    assert_eq!(SnailfishNumber::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3488);
}

#[test]
fn adds_and_magnitudes_according_to_examples () {
    let rows = [
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ];
    let numbers = rows.iter().map(|sn| SnailfishNumber::parse(*sn)).collect::<Vec<SnailfishNumber>>();

    let mut current = numbers[0].clone();
    current.reduce();
    for sn in numbers[1..].into_iter() {
        current = SnailfishNumber {
            left: Some(Box::new(current.clone())),
            right: Some(Box::new(sn.clone())),
            data: (0, 0)
        };
        current.reduce()
    }

    assert_eq!(current.to_string(), "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
    assert_eq!(current.magnitude(), 4140);
}

#[test]
fn ops_addition_according_to_examples () {

    let res = SnailfishNumber::parse("[[[[4,3],4],4],[7,[[8,4],9]]]") + SnailfishNumber::parse("[1,1]");
    assert_eq!(res.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

    let rhs = SnailfishNumber::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
    let lhs = SnailfishNumber::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");

    let mut current = rhs + lhs;
    assert_eq!(current.to_string(), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");

    current = current + SnailfishNumber::parse("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]");
    assert_eq!(current.to_string(), "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");

    current = current + SnailfishNumber::parse("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]");
    assert_eq!(current.to_string(), "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]");

    current = current + SnailfishNumber::parse("[7,[5,[[3,8],[1,4]]]]");
    assert_eq!(current.to_string(), "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]");

    current = current + SnailfishNumber::parse("[[2,[2,2]],[8,[8,1]]]");
    assert_eq!(current.to_string(), "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]");

    current = current + SnailfishNumber::parse("[2,9]");
    assert_eq!(current.to_string(), "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]");

    current = current + SnailfishNumber::parse("[1,[[[9,3],9],[[9,0],[0,7]]]]");
    assert_eq!(current.to_string(), "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]");

    current = current + SnailfishNumber::parse("[[[5,[7,4]],7],1]");
    assert_eq!(current.to_string(), "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]");

    current = current + SnailfishNumber::parse("[[[[4,2],2],6],[8,7]]");
    assert_eq!(current.to_string(), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
}

#[test]
fn sum_max_magnitude_according_to_examples () {
    let rows = [
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ];
    let numbers = rows.iter().map(|sn| SnailfishNumber::parse(*sn)).collect::<Vec<SnailfishNumber>>();

    assert_eq!(max_magnitude_of_sum(numbers), 3993);
}

fn main () {
    let file_content = fs::read_to_string("src\\day_18_input.txt").expect("Something went wrong reading the file");
    let snailfish_numbers = file_content.split("\r\n").map(|r| SnailfishNumber::parse(r)).collect::<Vec<SnailfishNumber>>();
    
    let mut current = snailfish_numbers[0].clone();
    current.reduce();
    for sn in snailfish_numbers[1..].into_iter() {
        current = SnailfishNumber {
            left: Some(Box::new(current.clone())),
            right: Some(Box::new(sn.clone())),
            data: (0, 0)
        };
        current.reduce()
    }

    println!("Addition magnitude: {:?}", current.magnitude());
    println!("Max magnitude: {:?}", max_magnitude_of_sum(snailfish_numbers));

}