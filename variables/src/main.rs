fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!(spaces);

    let spaces = "   ";
    let spaces = spaces.len(); // シャドーイング：覆い隠す
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
    // タプル
    let tup: (i32, f64, u8) = (200_000, 6.4, 1);
    let (tup_x, tup_y, tup_z) = tup; // 分配
    println!("The value of tup_y is: {}", tup_y);
    println!("The value of tup.0 is: {}", tup.0);

    // 配列
    let list_a = [1, 2, 3, 4, 5];
    let list_b: [i32; 5] = [1, 2, 3, 4, 5]; // i32型の要素が5つ
    let list_c = [3; 5]; // 3で初期化した配列が5つ
    println!("The value of list_a[0] is: {}", list_a[0]);
}
