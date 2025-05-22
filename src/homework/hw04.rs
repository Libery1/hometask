const HEIGHT: usize = 11;
const WIDTH: usize = 11;

fn main() {
    let mut output = String::new();
    let mid = HEIGHT / 2;

    for i in 0..HEIGHT {
        let stars = if i <= mid {
            2 * i + 1
        } else {
            2 * (HEIGHT - 1 - i) + 1
        };
        let spaces = (WIDTH - stars) / 2;

        for _ in 0..spaces {
            output.push(' ');
        }
        for _ in 0..stars {
            output.push('*');
        }
        for _ in 0..spaces {
            output.push(' ');
        }
        output.push('\n');
    }

    print!("{}", output);
}
