

fn recursive_fizzbuzz(state: i32) -> () {
    if state % 3 == 0 && state % 5 == 0 {
        println!("FizzBuzz!");
    } else if state % 3 == 0 {
        println!("Fizz");
    } else if state % 5 == 0 {
        println!("Buzz");
    }

    if state <= 100 {
        recursive_fizzbuzz(state + 1);
    }
}

fn main() {
    recursive_fizzbuzz(1);
}
