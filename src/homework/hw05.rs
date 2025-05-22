use std::io;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let a: u32 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let b: u32 = input2.trim().parse().expect("Please enter a valid number");

    let result = gcd(a, b);

    println!("GCD of {} and {} is {}", a, b, result);
}
