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
}
