use std::io;

fn is_prime(n: &u32) -> bool {
    match *n {
        0 | 1 => false,
        2 => true,
        _ if *n % 2 == 0 => false,
        _ => {
            let sqrt_n = (*n as f64).sqrt() as u32;
            (3..=sqrt_n).step_by(2).all(|i| *n % i != 0)
        }
    }
}

#[test]
fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];
    
    test_data.iter().for_each(|(n, prime)| {
        assert_eq!(is_prime(n), *prime)
    });
}

fn main() {
    println!("Введіть число для перевірки на простоту:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не вдалося прочитати ввід");
    
    match input.trim().parse::<u32>() {
        Ok(num) => {
            println!("Число {} {} простим", 
                    num, 
                    if is_prime(&num) { "є" } else { "не є" });
        },
        Err(_) => {
            println!("Будь ласка, введіть коректне ціле число!");
        }
    }
}