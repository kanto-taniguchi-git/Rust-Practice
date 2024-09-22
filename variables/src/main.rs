fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!(spaces);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // 型
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    let fast: i32 = 42; // 基準型
    println!("{}", fast);

    let y = 2.0; // f64(基準型)
    let z: f32 = 3.0; // f32

    // 演算
}
