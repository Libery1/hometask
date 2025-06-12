use std::io;

fn is_palindrome_number(mut n: u64) -> bool {
    if n < 10 {
        return true;
    }
    
    let original = n;
    let mut reversed = 0;
    
    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    
    original == reversed
}

fn main() {
    println!("Введіть число для перевірки на паліндром:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не вдалося прочитати рядок");
    
    let num: u64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Помилка: введіть коректне натуральне число!");
            return;
        }
    };
    
    println!("{} {} паліндромом", 
        num, 
        if is_palindrome_number(num) { "є" } else { "не є" });
}