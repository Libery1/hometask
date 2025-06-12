fn main() {
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;
    let mut perm = Vec::new();
    generate_permutations(&mut perm, &digits, &mut |p| {
        let m = p[0];
        let u = p[1];
        let x = p[2];
        let a = p[3];
        let s = p[4];
        let l = p[5];
        let o = p[6];
        let n = p[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            println!("{m}{u}{x}{a} * {a} = {s}{l}{o}{n}");
            count += 1;
        }
    });

    println!("Загальна кількість рішень: {count}");
}

fn generate_permutations<F>(current: &mut Vec<u32>, remaining: &Vec<u32>, callback: &mut F)
where
    F: FnMut(&Vec<u32>),
{
    if remaining.is_empty() {
        callback(current);
        return;
    }

    for i in 0..remaining.len() {
        let mut next_remaining = remaining.clone();
        let next_digit = next_remaining.remove(i);
        current.push(next_digit);
        generate_permutations(current, &next_remaining, callback);
        current.pop();
    }
}
