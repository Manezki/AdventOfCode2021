use std::fs;

fn main() {
    let file_content = fs::read_to_string("src\\day_7_input.txt").expect("Something went horribly wrong reading the file");
    let crab_sub_locations = file_content.split(",").map(|l| i64::from_str_radix(l, 10).unwrap()).collect::<Vec<i64>>();

    let mut sorted_subs = crab_sub_locations.clone();
    sorted_subs.sort();

    let median;

    if sorted_subs.len() % 2 == 0 {
        let (left_idx, right_idx) = (sorted_subs.len() /2, (sorted_subs.len() as f32 / 2.0).ceil() as usize);
        median = (sorted_subs[left_idx] + sorted_subs[right_idx]) / 2;
    } else {
        median = sorted_subs[sorted_subs.len() / 2];
    }

    let min_loc = *crab_sub_locations.iter().min().unwrap() as usize;
    let max_loc = *crab_sub_locations.iter().max().unwrap() as usize;

    println!("{:?}", min_loc);
    println!("{:?}", max_loc);

    let mut optimal_fuel = i64::max_value();

    println!("{:?}", optimal_fuel);

    for loc in min_loc..=max_loc {

        let mut fuel_used = 0;

        for sub in &crab_sub_locations {
            let steps = (loc as i64 - *sub).abs();
            fuel_used += (steps*(steps+1))/2;
        }

        if fuel_used < optimal_fuel {
            optimal_fuel = fuel_used;
        }

    }

    // let subs = crab_sub_locations.len() as f32;
    // let location_sum = crab_sub_locations.iter().sum::<i64>() as f32;
    let optimal_location = (median as f64- 0.5);

    let fuel_used: f64 = crab_sub_locations.iter().map(|sub| ((optimal_location - *sub as f64).abs()*((optimal_location - *sub as f64).abs() + 1 as f64))/2 as f64).sum();


    println!("{:?}", optimal_fuel);
    println!("{:?}", fuel_used);
}