use std::io;

fn main() {
    println!("Input the nth value you want to get from the fibonacci sequence");
    let mut nth = String::new();

    io::stdin()
        .read_line(&mut nth)
        .expect("Error reading line");

    let nth: u32 = nth
        .trim()
        .parse()
        .expect("Not a valid index");

    let nth_value = get_fib(nth);

    println!("value is {}", nth_value);
}

fn get_fib(x: u32) -> u32 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        let mut nth = 1;
        let mut index = 2;
        let mut prev = 1;
        let mut prev_prev = 0;

        while index < x {
            nth = prev + prev_prev;
            prev_prev = prev;
            prev = nth;
            index += 1;
        }

        return nth;
    }
}
