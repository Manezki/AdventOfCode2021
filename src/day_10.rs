use std::fs;

fn validate_row(row: &str) -> Option<usize> {
    let mut stack: Vec<char> = vec![];

    for c in row.chars() {
        match c {
            ')' | '>' | ']' | '}' => {
                let previous = stack.pop().unwrap_or_default();
                
                if [('(', ')'), ('<', '>'), ('[', ']'), ('{', '}')].contains(&(previous, c)) {
                    continue;
                } else {
                    match c {
                        ')' => return Some(3),
                        '>' => return Some(25137),
                        ']' => return Some(57),
                        '}' => return Some(1197),
                        _ => panic!("Unknown input char {}", previous)
                    }
                }
            },
            '(' | '<' | '[' | '{' => {
                stack.push(c);
            }
            _ => panic!("Unknown input char")
        }
    }
    return None;
}

fn fill_row(row: &str) -> Vec<char> {
    let mut stack: Vec<char> = vec![];

    for c in row.chars() {
        match c {
            ')' | '>' | ']' | '}' => {
                let previous = stack.pop().unwrap_or_default();
                
                if [('(', ')'), ('<', '>'), ('[', ']'), ('{', '}')].contains(&(previous, c)) {
                    continue;
                } else {
                    panic!("Corrupted line? {:?}", row);
                }
            },
            '(' | '<' | '[' | '{' => {
                stack.push(c);
            }
            _ => panic!("Unknown input char")
        }
    }

    let mut row_filler: Vec<char> = Vec::new();

    for c in stack.iter().rev() {
        match c {
            '(' => row_filler.push(')'),
            '[' => row_filler.push(']'),
            '{' => row_filler.push('}'),
            '<' => row_filler.push('>'),
            _ => panic!("Unknown input char. {:?}", c)
        }
    }

    return row_filler;
}

fn autocomplete_score(autocomplete: Vec<char>) -> usize {
    let mut total = 0 as usize;

    for c in autocomplete {
        
        let addition: usize;
        match c {
            ')' => addition = 1,
            ']' => addition = 2,
            '}' => addition = 3,
            '>' => addition = 4,
            _ => panic!("Unknown input char. {:?}", c)
        }

        total = (total * 5) + addition;
    }

    return total;
}

fn main() {
    let file_content = fs::read_to_string("src\\day_10_input.txt").expect("Something went wrong reading the file");
    let input_rows = file_content.split("\r\n").collect::<Vec<&str>>();

    let mut illegal_char_sum = 0;

    let valid_rows = input_rows.iter().map(|r| *r).filter(|r| {
        let res = validate_row(*r);

        if res.is_none() {
            return true;
        } else {
            illegal_char_sum += res.unwrap() as usize;
            return false;
        }

    }).collect::<Vec<&str>>();

    let mut autocomplete_scores: Vec<usize> = Vec::new();

    for row in valid_rows {
        let filler = fill_row(row);
        autocomplete_scores.push(autocomplete_score(filler));
    }

    autocomplete_scores.sort();

    println!("{:?}", illegal_char_sum);
    println!("{:?}", autocomplete_scores[autocomplete_scores.len() >> 1]);
}

#[test]
fn rejects_incorrect_row() {

    let inputs = vec![
        "{([(<{}[<>[]}>{[]{[(<()>",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{"
    ];

    for input in inputs {
        assert_eq!(validate_row(input).is_none(), false);
    }
}

#[test]
fn returns_points_for_incorrect_character() {

    let inputs = vec![
        ("{([(<{}[<>[]}>{[]{[(<()>", 1197),
        ("[[<[([]))<([[{}[[()]]]", 3),
        ("[{[{({}]{}}([{[{{{}}([]", 57),
        ("[<(<(<(<{}))><([]([]()", 3),
        ("<{([([[(<>()){}]>(<<{{", 25137)
    ];
    
    for (input, points) in inputs {
        assert_eq!(validate_row(input).unwrap(), points as usize);
    }
}

#[test]
fn accepts_valid_input_row() {

    let inputs = vec![
        "[({(<(())[]>[[{[]{<()<>>",
    ];
    
    for input in inputs {
        assert_eq!(validate_row(input).is_none(), true);
    }
}

#[test]
fn fill_row_confers_to_example() {

    let inputs = vec![
        "[({(<(())[]>[[{[]{<()<>>",
    ];
    
    for input in inputs {
        assert_eq!(fill_row(input), "}}]])})]".chars().collect::<Vec<char>>());
    }
}

#[test]
fn autocomplete_score_confers_to_example() {
    assert_eq!(autocomplete_score("}}]])})]".chars().collect::<Vec<char>>()), 288957);
}
