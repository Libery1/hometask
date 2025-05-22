use std::io;

fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[test]
fn test() {
    let data = [
        ("Hello", "hELLO"),
        ("Привет", "пРИВЕТ"),
    ];
    
    data.iter().for_each(|(a, b)| {
        assert_eq!(invert_the_case(a.to_string()), b.to_string());
        assert_eq!(invert_the_case(b.to_string()), a.to_string());
    });
}

fn main() {
    println!("Введіть текст для інвертування регістру:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не вдалося прочитати ввід");
    
    let input = input.trim_end().to_string();
    
    let inverted = invert_the_case(input);
    println!("Результат: {}", inverted);
}