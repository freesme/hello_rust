fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions: [&str; 3] = [southern_germany, chinese, english];
    for ele in regions {
        println!("{}", ele)
    }
}

fn main() {
    greet_world();
}
