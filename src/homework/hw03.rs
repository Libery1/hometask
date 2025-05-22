const WIDTH: usize = 30;
const HEIGHT: usize = 15;

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ch = if y == 0 || y == HEIGHT - 1 {
                '*'
            } else if x == 0 || x == WIDTH - 1 {
                '*'
            } else if x == y * 2 || x == WIDTH - 1 - y * 2 {
                '*'
            } else if y >= HEIGHT / 1 && (x == 2 * (HEIGHT - 1 - y) || x == WIDTH - 1 - 2 * (HEIGHT - 1 - y)) {
                '*'
            } else {
                ' '
            };
            output.push(ch);
        }
        output.push('\n');
    }

    print!("{}", output);
}
