use std::{fs, collections::HashMap, result};

fn apply_replacements (template: String, rules: &HashMap<&str, String>) -> String {

    let steps = template.len();

    let mut replaced_string = template.clone();
    let mut chars_added = 0 as usize;

    for i in 0..=steps-2 {
        let replacement_idx = i + chars_added;
        let template_slice = &replaced_string[replacement_idx..replacement_idx+2];

        if rules.contains_key(template_slice) {

            let left = replaced_string[..replacement_idx].to_string();
            let mid = rules[template_slice].clone();
            let right = replaced_string[replacement_idx+2..].to_string();
            
            // println!("Template {:?}", replaced_string);
            // println!("Matched {:?} to {:?}", template_slice, mid);
            // println!("{:?}, {:?}, {:?}", left, mid, right);

            replaced_string = String::from_iter([left, mid, right]);
            chars_added += 1;

            // Only one rule should apply to each position
            continue;
        }
    }

    return replaced_string;
}

#[test]
fn applying_rules_confers_to_example() {
    let mut template = "NNCB".to_string();
    let rules: HashMap<&str, String> = HashMap::from_iter([("CH", "CBH".to_string()),
        ("HH", "HNH".to_string()),
        ("CB", "CHB".to_string()),
        ("NH", "NCH".to_string()),
        ("HB", "HCB".to_string()),
        ("HC", "HBC".to_string()),
        ("HN", "HCN".to_string()),
        ("NN", "NCN".to_string()),
        ("BH", "BHH".to_string()),
        ("NC", "NBC".to_string()),
        ("NB", "NBB".to_string()),
        ("BN", "BBN".to_string()),
        ("BB", "BNB".to_string()),
        ("BC", "BBC".to_string()),
        ("CC", "CNC".to_string()),
        ("CN", "CCN".to_string()),]);

    let results = [
        "NCNBCHB".to_string(),
        "NBCCNBBBCBHCB".to_string(),
        "NBBBCNCCNBBNBNBBCHBHHBCHB".to_string(),
        "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_string()
    ];

    for i in 0..results.len() {
        let res = apply_replacements(template.clone(), &rules);
        assert_eq!(res, results[i]);
        println!("{:?} OK", i);
        template = res;
    }
}


fn expand_letter_pairs (letter_pairs: HashMap<String, u64>, rules: &HashMap<&str, String>) -> HashMap<String, u64> {

    let mut resulting_pairs: HashMap<String, u64> = HashMap::new();

    for (pair, counts) in &letter_pairs {
        let maybe_expanded = rules.get(&pair[0..2]);
        
        if maybe_expanded.is_none() {
            resulting_pairs.insert(pair.to_string(), 1);
            // Should only happen with start and end character which are already contained in 'cloned_pairs'
            continue;
        }

        let expanded = maybe_expanded.unwrap();

        let (left, right) = (
            String::from_iter([pair.chars().nth(0).unwrap(), expanded.chars().nth(1).unwrap()]),
            String::from_iter([expanded.chars().nth(1).unwrap(), pair.chars().nth(1).unwrap()])
        );

        let left_count = *resulting_pairs.get(&left).unwrap_or(&0);
        resulting_pairs.insert(left, left_count + counts);

        let right_count = *resulting_pairs.get(&right).unwrap_or(&0);
        resulting_pairs.insert(right, right_count + counts);
    }

    return resulting_pairs;
}

#[test]
fn pair_count_confers_to_example() {
    let template = "_NNCB_".to_string();
    let rules: HashMap<&str, String> = HashMap::from_iter([("CH", "CBH".to_string()),
        ("HH", "HNH".to_string()),
        ("CB", "CHB".to_string()),
        ("NH", "NCH".to_string()),
        ("HB", "HCB".to_string()),
        ("HC", "HBC".to_string()),
        ("HN", "HCN".to_string()),
        ("NN", "NCN".to_string()),
        ("BH", "BHH".to_string()),
        ("NC", "NBC".to_string()),
        ("NB", "NBB".to_string()),
        ("BN", "BBN".to_string()),
        ("BB", "BNB".to_string()),
        ("BC", "BBC".to_string()),
        ("CC", "CNC".to_string()),
        ("CN", "CCN".to_string()),]);

    let results: Vec<HashMap<String, u64>> = vec![HashMap::from_iter([("_N".to_string(), 1), ("NC".to_string(), 1), ("CN".to_string(), 1),
        ("NB".to_string(), 1), ("BC".to_string(), 1), ("CH".to_string(), 1), ("HB".to_string(), 1), ("B_".to_string(), 1)])];

    let mut letter_pairs: HashMap<String, u64> = HashMap::new();
    for i in 1..template.len() { 
        letter_pairs.insert(template[i-1..i+1].to_string(), 1);
    }

    for i in 0..results.len() {
        let res = expand_letter_pairs(letter_pairs, &rules);
        assert_eq!(res, results[i]);
        letter_pairs = res;
    }
}

fn count_chars (letter_pairs: HashMap<String, u64>) -> HashMap<char, u64> {
    let mut counts: HashMap<char, u64> = HashMap::new();

    for (key, mult) in letter_pairs {
        if key.chars().nth(1) == Some('_') {
            continue;
        }
        let prev_count = *counts.get(&key.chars().nth(1).unwrap()).unwrap_or(&0);
        counts.insert(key.chars().nth(1).unwrap(), prev_count + mult);
    }

    return counts;
}

#[test]
fn char_count_confers_to_example() {
    let mut letter_pairs: HashMap<String, u64> = HashMap::new();
    let exp_template = "_NNCB_";

    let rules: HashMap<&str, String> = HashMap::from_iter([("CH", "CBH".to_string()),
        ("HH", "HNH".to_string()),
        ("CB", "CHB".to_string()),
        ("NH", "NCH".to_string()),
        ("HB", "HCB".to_string()),
        ("HC", "HBC".to_string()),
        ("HN", "HCN".to_string()),
        ("NN", "NCN".to_string()),
        ("BH", "BHH".to_string()),
        ("NC", "NBC".to_string()),
        ("NB", "NBB".to_string()),
        ("BN", "BBN".to_string()),
        ("BB", "BNB".to_string()),
        ("BC", "BBC".to_string()),
        ("CC", "CNC".to_string()),
        ("CN", "CCN".to_string()),]);

    for i in 1..exp_template.len() {
        letter_pairs.insert(exp_template[i-1..i+1].to_string(), 1);
    }

    for _ in 0..40 {
        letter_pairs = expand_letter_pairs(letter_pairs, &rules);
    }


    let counts = count_chars(letter_pairs);

    assert_eq!((counts.values().max().unwrap() - counts.values().min().unwrap()), 2188189693529);
}


fn main () {
    let file_content = fs::read_to_string("src\\day_14_input.txt").expect("Something went wrong reading the file");
    
    let mut input_parts = file_content.split("\r\n\r\n");
    let (template, raw_rules) = (input_parts.next().unwrap().to_string(), input_parts.next().unwrap());
    let rules: HashMap<&str, String> = HashMap::from_iter(raw_rules.split("\r\n")
        .map(|rule| {
            let mut parts = rule.split(" -> ");
            let (from, addition) = (parts.next().unwrap(), parts.next().unwrap());
            let to = String::from_iter([from.chars().nth(0).unwrap(), addition.chars().nth(0).unwrap(), from.chars().nth(1).unwrap()]);
            return (from, to)
        }));

    let mut mutable_template = template.clone();

    for i in 0..10 {
        mutable_template = apply_replacements(mutable_template, &rules);
    }

    let mut counts: HashMap<char, u64> = HashMap::new();

    mutable_template.chars().for_each(|c| {
        let count = *counts.entry(c).or_insert(0);
        counts.insert(c, count + 1);
        ();
    });

    // 10 steps a.k.a. 1 star:
    // Most common 4004, least common 598
    // 4004 - 598 = 3406

    println!("{:?}", counts);

    // Puzzle 2

    let mut letter_pairs: HashMap<String, u64> = HashMap::new();
    let exp_template = "_".to_string() + &template.to_string() + &"_".to_string();

    for i in 1..exp_template.len() {
        let prev = *letter_pairs.get(&exp_template[i-1..i+1].to_string()).unwrap_or(&0);
        letter_pairs.insert(exp_template[i-1..i+1].to_string(), prev + 1);
    }

    println!("{:?}", letter_pairs);

    for _ in 0..40 {
        letter_pairs = expand_letter_pairs(letter_pairs, &rules);
    }


    let counts = count_chars(letter_pairs);

    println!("{:?}", (counts.values().max().unwrap() - counts.values().min().unwrap()));
}