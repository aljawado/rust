use std::io;

fn main() {

    loop {
        let mut input = String::new();
        println!("enter number: ");
        io::stdin().read_line(&mut input)
            .expect("Error reading input");

        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("fib for {} is {}", number, calc_fib(number));
    }
}

fn calc_fib(x: i32) -> i32{
    if x==1 {
        return 1;
    }
    x + calc_fib(x-1)
}

