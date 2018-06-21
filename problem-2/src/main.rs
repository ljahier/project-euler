fn main() {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 0;
    let mut result: i32 = 0;

    while b <= 4000000 {
        match b % 2 {
            0 => result += b,
            _ => print!("")
        }
        c = a;
        a = b;
        b = c + b
    }
    println!("{:?}", result);
}
