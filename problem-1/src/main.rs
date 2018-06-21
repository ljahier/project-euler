fn main() {
    let mut result: i32 = 0;
    for i in 1..1000 {
        match (i % 3, i % 5) {
            (0, _) => result+= i,
            (_, 0) => result+= i,
            _ => print!(""),
        };
    }
    println!("{:?}", result);
}
