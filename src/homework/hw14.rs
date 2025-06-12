use std::collections::HashSet;

fn gray(n: u8) -> Vec<String> {
    match n {
        0 => vec!["".to_string()],
        _ => {
            let mut result = Vec::new();
            let prev_gray = gray(n - 1);
            
            for code in &prev_gray {
                result.push(format!("0{}", code));
            }
            
            for code in prev_gray.iter().rev() {
                result.push(format!("1{}", code));
            }
            
            result
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Rectangle {
            top_left: Point { 
                x: x1.min(x2), 
                y: y1.max(y2) 
            },
            bottom_right: Point { 
                x: x1.max(x2), 
                y: y1.min(y2) 
            },
        }
    }

    fn iter_points(&self) -> impl Iterator<Item = Point> + '_ {
        (self.top_left.x..self.bottom_right.x).flat_map(move |x| {
            (self.bottom_right.y..self.top_left.y).map(move |y| Point { x, y })
        })
    }
}

fn area_occupied(rects: &[Rectangle]) -> i32 {
    let unique_points: HashSet<Point> = rects
        .iter()
        .flat_map(|r| r.iter_points())
        .collect();
    
    unique_points.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gray_code() {
        let test_cases = vec![
            (0, vec![""]),
            (1, vec!["0", "1"]),
            (2, vec!["00", "01", "10", "11"]),
            (3, vec!["000", "001", "010", "011", 
                    "100", "101", "110", "111"]),
        ];
        
        for (n, expected) in test_cases {
            assert_eq!(gray(n), expected);
        }
    }
    
    #[test]
    fn test_area_occupied() {
        let rectangles = vec![
            Rectangle::new(2, 9, 5, 3),  // Площа: 18
            Rectangle::new(1, 8, 11, 6), // Площа: 20
            Rectangle::new(9, 10, 13, 2), // Площа: 32
        ];
        assert_eq!(area_occupied(&rectangles), 60);
    }
}

fn main() {
    println!("Код Грея для n=3:");
    for code in gray(3) {
        println!("{}", code);
    }
    
    let rectangles = vec![
        Rectangle::new(2, 9, 5, 3),
        Rectangle::new(1, 8, 11, 6),
        Rectangle::new(9, 10, 13, 2),
    ];
    
    println!("\nЗайнята площа: {}", area_occupied(&rectangles));
}