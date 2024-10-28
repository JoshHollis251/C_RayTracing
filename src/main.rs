use rand::Rng;

struct Point {
    x: f64,
    y: f64,
}

fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!"); //cringe
    let p = Point { x: 0.0, y: 0.0 };
    for i in 0..10 {
        //print a random number
        let x = rand::thread_rng().gen_range(-10.0..10.0);
        let newPoint = Point { x: rand::thread_rng().gen_range(-10.0..10.0), y: rand::thread_rng().gen_range(-10.0..10.0)};
        println!("coords: ({}, {})", newPoint.x, newPoint.y);
    }
}