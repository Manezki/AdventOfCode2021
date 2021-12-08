use std::fs;

fn main() {
    let file_content = fs::read_to_string("src\\day_8_input.txt").expect("Error reading the file");
    let inputs = file_content.split("\r\n").map(|l| {
        let mut l_split = l.split("|");
        return(l_split.next().unwrap(), l_split.next().unwrap());
    }).collect::<Vec<(&str, &str)>>();
    let (signals, outputs): (Vec<&str>, Vec<&str>) = inputs.iter().cloned().unzip();

    let mut count_1478 = 0;

    let mut total_sum = 0;

    for (input_signal, output_digits) in signals.iter().zip(outputs.iter()) {
        let mut digit_to_signals: Vec<Vec<char>> = vec![vec![]; 10];
        
        let digit_signals = input_signal.split(" ").collect::<Vec<&str>>();
        for signal in &digit_signals {
            match signal.len() {
                2 => digit_to_signals[1] = signal.chars().collect(),
                4 => digit_to_signals[4] = signal.chars().collect(),
                3 => digit_to_signals[7] = signal.chars().collect(),
                7 => digit_to_signals[8] = signal.chars().collect(),
                _ => (),
            }
        }

        let lower_left_and_bottom: Vec<char> = digit_to_signals[8].iter().cloned().filter(|c| {
            !digit_to_signals[4].contains(c) & !digit_to_signals[7].contains(c)
        }).collect();

        digit_to_signals[2] = digit_signals.iter().cloned().find(|s| {
            (s.len() == 5) & s.chars().collect::<Vec<char>>().contains(&lower_left_and_bottom[0]) & s.chars().collect::<Vec<char>>().contains(&lower_left_and_bottom[1])
        }).unwrap().chars().collect();

        digit_to_signals[9] = digit_signals.iter().cloned().find(|s| {
            if s.len() != 6 {
                return false;
            }
            let s_char = s.chars().collect::<Vec<char>>();
            let remainder = digit_to_signals[8].iter().filter(|c| !s_char.contains(c)).next();


            return !(digit_to_signals[4].contains(remainder.unwrap()))
        }).unwrap().chars().collect();

        digit_signals.iter().for_each(|s| {
            if s.len() != 6 {
                return ();
            }

            let s_char = s.chars().collect::<Vec<char>>();

            if digit_to_signals[9].iter().filter(|c| !s_char.contains(c)).next().is_none() {
                return ();
            }

            let remainder = digit_to_signals[8].iter().filter(|c| !s_char.contains(c)).next();

            if digit_to_signals[1].contains(remainder.unwrap()) {
                digit_to_signals[6] = s_char;
            } else {
                digit_to_signals[0] = s_char;
            }

        });

        digit_signals.iter().for_each(|s| {
            if s.len() != 5 {
                return ();
            }

            let s_char = s.chars().collect::<Vec<char>>();

            if digit_to_signals[2].iter().filter(|c| !s_char.contains(c)).next().is_none() {
                return ();
            }

            let remainder = digit_to_signals[6].iter().filter(|c| !s_char.contains(c)).collect::<Vec<&char>>();

            if remainder.len() == 2 {
                digit_to_signals[3] = s_char;
            } else {
                digit_to_signals[5] = s_char;
            }
        });

        for (mult, digit) in output_digits.trim().split(" ").map(|d| d.trim().chars().collect::<Vec<char>>()).enumerate() {
            
            let multiple = (10 as i32).pow(3-mult as u32);

            let mut found = false;

            for (key, signal) in digit_to_signals.iter().enumerate() {
                if (digit.len() == signal.len()) & (digit.iter().all(|c| signal.contains(&c))) {
                    total_sum += multiple*key as i32;
                    found = true;
                    break;
                }
            }

            if !found | (digit.len() < 2) | !vec![1000, 100, 10, 1].contains(&multiple) {
                println!("No match");
                println!("{:?}", digit_to_signals);
                println!("{:?}", digit);
                std::panic::panic_any("!");
            }
        }
    }

    for output in outputs {
        let digits = output.split(" ");
        for digit in digits {
            match digit.len() {
                2 | 4 | 3 | 7 => count_1478 += 1 as i32,
                _ => (),
            }
        }
    }



    println!("{}", count_1478);
    println!("{}", total_sum);
    // Not 10436970
}