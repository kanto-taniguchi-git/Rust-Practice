fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {}", x);

    let y = plus_one(10);
    println!("The value of y is: {}", y);

    let z = {
        let zz = 3;
        zz + 1
    };

    println!("The value of z is: {}", z);
}
fn another_function(x :i32) { // 型の宣言必須
    println!("Another function.");
    println!("The value of x is: {}", x);
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}

// 戻り値のある関数
fn five() -> i32 {
    5
}

fn plus_one(y: i32) -> i32 {
    y + 1
}

