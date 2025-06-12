use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let ships1 = vec![1, 1, 1, 1, 6];
    print_solution(&ships1);
    
    let ships2 = vec![8, 2, 2, 4, 4];
    print_solution(&ships2);
    
    let ships3 = vec![1, 2, 3];
    print_solution(&ships3);
    
    let test_ships = generate_valid_ships(5);
    println!("\nGenerated ships: {:?}", test_ships);
    print_solution(&test_ships);
}

fn count_moves(ships: &[u32]) -> i32 {
    let total: u32 = ships.iter().sum();
    let n = ships.len() as u32;
    
    if total % n != 0 {
        return -1;
    }
    
    let target = total / n;
    let mut moves: i32 = 0;
    let mut current_sum: i32 = 0;
    
    for (i, &weight) in ships.iter().enumerate() {
        current_sum += weight as i32;
        let expected_sum = (target * (i as u32 + 1)) as i32;
        moves += (current_sum - expected_sum).abs();
    }
    
    moves / 2
}

fn print_solution(ships: &[u32]) {
    println!("\nInitial distribution: {:?}", ships);
    match count_moves(ships) {
        -1 => println!("Cannot balance the cargo!"),
        m => println!("Minimum moves required: {}", m),
    }
}

struct SimpleRng(u32);

impl SimpleRng {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        SimpleRng(seed)
    }
    
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0
    }
    
    fn gen_range(&mut self, low: u32, high: u32) -> u32 {
        low + self.next() % (high - low)
    }
}

fn generate_valid_ships(n: usize) -> Vec<u32> {
    let mut rng = SimpleRng::new();
    let target = rng.gen_range(10, 20);
    let mut ships = vec![target; n];
    
    for i in 0..n/2 {
        let delta = rng.gen_range(1, 5);
        ships[i] += delta;
        ships[n-i-1] = ships[n-i-1].saturating_sub(delta);
    }
    
    ships
}