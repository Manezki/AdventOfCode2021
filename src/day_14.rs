use std::{fs, collections::HashMap};

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

    let steps = 40;
    let mut mutable_template = template.clone();

    for i in 0..steps {
        println!("{:?}", i);
        mutable_template = apply_replacements(mutable_template, &rules);
    }

    let mut counts: HashMap<char, u32> = HashMap::new();

    mutable_template.chars().for_each(|c| {
        let count = *counts.entry(c).or_insert(0);
        counts.insert(c, count + 1);
        ();
    });

    // 10 steps a.k.a. 1 star:
    // Most common 4004, least common 598
    // 4004 - 598 = 3406
    println!("{:?}", counts);
}