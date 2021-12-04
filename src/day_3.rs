use std::fs;

fn filter_oxygen_by_position<>(lines: Vec<String>, position: u8) -> String {
    // let lines = lines_to_copy.clone();

    let mut count_ones = 0;
    let mut count_zeros = 0;

    for line in &lines {
        if line.chars().map(|c| c.to_string()).collect::<Vec<String>>()[position as usize] == String::from("1") {
            count_ones += 1;
        } else {
            count_zeros += 1;
        }
    }

    let more_common = if count_ones >= count_zeros { String::from("1") } else { String::from("0") };

    let mut bit_filtered_lines: Vec<String> = Vec::new();
    
    for line in &lines {
        if line.chars().map(|c| c.to_string()).collect::<Vec<String>>()[position as usize] == more_common {
            bit_filtered_lines.push(line.clone());
        }
    }

    if bit_filtered_lines.len() == 1 {
        println!("{:?}", bit_filtered_lines);
        return bit_filtered_lines[0].clone();
    } else {
        return filter_oxygen_by_position(bit_filtered_lines, position + 1);        
    }
}

fn filter_co2_by_position<>(lines: Vec<String>, position: u8) -> String {
    // let lines = lines_to_copy.clone();

    let mut count_ones = 0;
    let mut count_zeros = 0;

    for line in &lines {
        if line.chars().map(|c| c.to_string()).collect::<Vec<String>>()[position as usize] == String::from("1") {
            count_ones += 1;
        } else {
            count_zeros += 1;
        }
    }

    let less_common = if count_ones >= count_zeros { String::from("0") } else { String::from("1") };

    let mut bit_filtered_lines: Vec<String> = Vec::new();
    
    for line in &lines {
        if line.chars().map(|c| c.to_string()).collect::<Vec<String>>()[position as usize] == less_common {
            bit_filtered_lines.push(line.clone());
        }
    }

    if bit_filtered_lines.len() == 1 {
        println!("{:?}", bit_filtered_lines);
        return bit_filtered_lines[0].clone();
    } else {
        return filter_co2_by_position(bit_filtered_lines, position + 1);
    }
}

fn main() {
    let content = fs::read_to_string("C:\\Users\\mande\\Projects\\Rust-test\\src\\day_3_input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<String> = content.split("\r\n").map(|l| l.to_string()).collect();

    let mut line_count: u32 = 1;
    let mut counts: Vec<u32> = Vec::new();

    let first_line = lines.iter().next().unwrap();
    
    for bit in first_line.chars() {
        counts.push(bit.to_digit(10).unwrap());
    }

    for line in &lines {
        for (i, bit) in line.chars().map(|b| b.to_digit(10).unwrap()).enumerate() {
            counts[i] = counts[i] + bit;
        }
        line_count = line_count + 1;
    }
    
    let gamma = counts.iter().map(|c| if c > &(&line_count/2) {1} else {0}).collect::<Vec<u32>>();
    let epsilon = counts.iter().map(|c| if c < &(&line_count/2) {1} else {0}).collect::<Vec<u32>>();

    let gamma_binary = gamma.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("");
    let epsilon_binary = epsilon.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("");

    let g = isize::from_str_radix(&gamma_binary, 2).unwrap();
    let e = isize::from_str_radix(&epsilon_binary, 2).unwrap();

    println!("{:?}", g*e);

    // lines.iter().for_each(|l| println!("{}", l));

    let oxygen_binary = filter_oxygen_by_position(lines, 0);
    let o = isize::from_str_radix(&oxygen_binary, 2).unwrap();

    let co2_binary = filter_co2_by_position(content.split("\r\n").map(|l| l.to_string()).collect(), 0);
    let co2 = isize::from_str_radix(&co2_binary, 2).unwrap();

    println!("{:?}", o*co2);
}
