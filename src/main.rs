struct Point {
    x: f64,
    y: f64,
}

fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
    let p = Point { x: 0.0, y: 0.0 };
    println!("The point is at ({}, {})", p.x, p.y);
}