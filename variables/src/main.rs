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

    let _y = 2.0; // f64(基準型)
    let _z: f32 = 3.0; // f32

    // 演算
    let sum = 5 + 10;
    println!("足し算の結果: {}", sum);

    let difference = 15.5 - 10.2;
    println!("引き算の結果: {}", difference);

    let product = 120 * 24;
    println!("掛け算の結果: {}", product);

    let quotient = 56.7 / 32.2;
    println!("割り算の結果: {}", quotient);
    let floored = 2 / 3;
    println!("割り算の小数部分の取り扱い: {}", floored);

    let remainder = 43 % 5;
    println!("剰余の結果: {}", remainder);

    // 論理値
    let _t: bool = true;
    let _f: bool = false;

    // 文字型
    let _c = 'z';
    let _z = 'Z';
    let _enjinner = '🧑';
    let _byte_data = b'a';
    println!("{}, {}, {}, {}", _c, _z, _enjinner, _byte_data);

    // 複合型
    
}
