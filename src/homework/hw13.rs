use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,  // верхній лівий кут
    b: Point,  // нижній правий кут
}

impl Rectangle {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Rectangle {
            a: Point { x: x1.min(x2), y: y1.max(y2) },  // нормалізуємо координати
            b: Point { x: x1.max(x2), y: y1.min(y2) },
        }
    }

    fn area(&self) -> i32 {
        (self.b.x - self.a.x) * (self.a.y - self.b.y)
    }

    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for x in self.a.x..self.b.x {
            for y in self.b.y..self.a.y {
                points.push(Point { x, y });
            }
        }
        points
    }
}

fn area_occupied(rects: &[Rectangle]) -> i32 {
    let mut unique_points = HashSet::new();
    
    for rect in rects {
        for point in rect.points() {
            unique_points.insert(point);
        }
    }
    
    unique_points.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle::new(2, 9, 5, 3),  // червоний
        Rectangle::new(1, 8, 11, 6),  // зелений
        Rectangle::new(9, 10, 13, 2), // синій
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    let rectangles = test_data();
    let total_area: i32 = rectangles.iter().map(|r| r.area()).sum();
    let occupied_area = area_occupied(&rectangles);
    
    println!("Сума площ окремих прямокутників: {}", total_area);
    println!("Фактично зайнята площа: {}", occupied_area);
}