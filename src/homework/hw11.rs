use std::time::{SystemTime, UNIX_EPOCH};

fn gen_random_vector(n: usize) -> Vec<i32> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    let mut rng = LcgRng::new(seed);
    (0..n).map(|_| rng.gen_range(10, 100)).collect()
}

struct LcgRng(u32);

impl LcgRng {
    fn new(seed: u32) -> Self {
        LcgRng(seed)
    }

    fn gen_range(&mut self, low: i32, high: i32) -> i32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        let range = (high - low) as u32;
        low + (self.0 % range) as i32
    }
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    data.windows(2)
        .enumerate()
        .map(|(i, pair)| (pair[0] + pair[1], i, i + 1))
        .min_by_key(|&(sum, _, _)| sum)
        .unwrap()
}

fn print_result(data: &[i32], min_sum: i32, i1: usize, i2: usize) {
    println!("indexes: {}", (0..data.len()).map(|i| format!("{}.", i)).collect::<Vec<_>>().join(" "));
    println!("data: {:?}", data);
    println!("indexes: \\__ __/");
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[i1], data[i2], min_sum, i1, i2);
}

fn main() {
    let data = gen_random_vector(20);
    let (min_sum, i1, i2) = min_adjacent_sum(&data);
    print_result(&data, min_sum, i1, i2);
}