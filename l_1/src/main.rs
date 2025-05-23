fn main() {
    greet_world();
}

fn greet_world() {
    let english = "Hello World!";
    let german = "Grüß Gott!";
    let japanese = "こんにちは、世界よ！";
    let regions = [german, japanese, english];
    for region in regions.iter() {
        println!("{}", region);
    }

    println!("");

    for region in regions.iter().rev() {
        println!("{}", region);
    }
}
