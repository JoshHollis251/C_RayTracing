use rand::Rng;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

static MAGIC_NUMBER: i32 = 30;

fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!"); //cringe
    for i in 0..10 {
        //print a random number
        let new_point = Point { x: rand::thread_rng().gen_range(-10.0..10.0), y: rand::thread_rng().gen_range(-10.0..10.0)};
        println!("coords: ({}, {}) {}", new_point.x, new_point.y, i);
    }
}