fn draw_christmas_tree(triangles: usize) {
    let max_width = 2 * triangles - 1;
    
    for triangle in 1..=triangles {
        for row in 1..=triangle + 1 {
            let stars = 2 * row - 1;
            let spaces = (max_width - stars) / 2;
            
            (0..spaces).for_each(|_| print!(" "));
            (0..stars).for_each(|_| print!("*"));
            (0..spaces).for_each(|_| print!(" "));
            
            println!();
        }
    }
    
    for _ in 0..triangles {
        let spaces = (max_width - 1) / 2;
        (0..spaces).for_each(|_| print!(" "));
        print!("*");
        (0..spaces).for_each(|_| print!(" "));
        println!();
    }
}
fn main() {
    let triangles = 5;
    draw_christmas_tree(triangles);
}