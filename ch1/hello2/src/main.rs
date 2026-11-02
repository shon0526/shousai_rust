fn greet_world() {
    println!("Hello, World");
    let southern_germany = "GrubGott!";
    let japan = "ハローワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
fn main() {
    greet_world();
}
