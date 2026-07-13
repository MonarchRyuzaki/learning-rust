use std::io;

fn main() {
    println!("N = ?");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to get a input");

    let number = n.trim().parse().expect("Invalid Number");

    let nthfib = fibn(number);

    println!("The {number}th Fibonacci Number is {nthfib}");
}

fn fibn(number: i32) -> i32 {
    if number <= 1 {
        return number;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _i in { 2..number + 1 } {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}
