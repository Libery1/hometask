fn draw_tree(levels: usize) {
    let max_width = 1 + 2 * (levels + 1);

    for triangle in 1..=levels {
        let space = " ".repeat((max_width - 1) / 2);
        println!("{}*", space);

        (0..triangle).for_each(|i| {
            let stars = "*".repeat(1 + 2 * i);
            let padding = " ".repeat((max_width - stars.len()) / 2);
            println!("{}{}", padding, stars);
        });
    }
}

fn main() {
    let triangle_count = 5;
    draw_tree(triangle_count);
}
