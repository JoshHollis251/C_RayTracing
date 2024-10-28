//TODO: implement raytracing
static IS_RAYTRACING: bool = true;
fn main() {
    if IS_RAYTRACING {
        println!("raytracing");
    } else {
        println!("not raytracing");
    }
}