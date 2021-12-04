use std::fs;

fn main() {
    let content = fs::read_to_string("C:\\Users\\mande\\Projects\\Rust-test\\src\\day_2_input.txt").expect("Something went wrong reading the file");
    let commands = content.split("\r\n");
    
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in commands {
        let mut splitter = cmd.split(" ");
        let direction = splitter.next().unwrap();
        let units = splitter.next().unwrap().parse::<i32>().unwrap();

        println!("{}", cmd);

        if direction == "forward" {
            x = x + units;
            depth = depth + aim * units;
        } else if direction == "down" {
            aim = aim + units;
        } else if direction == "up" {
            aim = aim - units;
        }
    }

    let product = x * depth;
    println!("{}", product);
    
}