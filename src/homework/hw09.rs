use std::io;

fn rotate(s: String, n: isize) -> String {
    if s.is_empty() {
        return s;
    }

    let len = s.len() as isize;
    let n = n % len;
    
    if n == 0 {
        return s;
    }

    let split_pos = if n > 0 {
        (len - n) as usize
    } else {
        (-n) as usize
    };

    let (left, right) = s.split_at(split_pos);
    format!("{}{}", right, left)
}

fn main() {
    println!("Введіть рядок для зсуву:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не вдалося прочитати рядок");
    let input_string = input_string.trim().to_string();

    println!("Введіть величину зсуву (ціле число, можна від'ємне):");
    let mut shift = String::new();
    io::stdin().read_line(&mut shift).expect("Не вдалося прочитати зсув");
    
    match shift.trim().parse::<isize>() {
        Ok(n) => {
            let result = rotate(input_string.clone(), n);
            println!("\nРезультат:");
            println!("Оригінальний рядок: \"{}\"", input_string);
            println!("Зсув на {} позиції: \"{}\"", n, result);
        },
        Err(_) => {
            println!("«Помилка! Будь ласка, введіть ціле число для зсуву.»");
        }
    }
}

#[test]
fn test_rotate() {
    let s = "abcdefgh";
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];
    
    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(rotate(s.to_string(), *n), exp.to_string());
    });
}