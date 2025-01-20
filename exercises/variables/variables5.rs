// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut number = "T-H-R-E-E"; // 添加mut使变量可变
    println!("Spell a Number : {}", number);
    number = "3"; // 保持字符串类型
    println!("Number plus two is : {}", number.parse::<i32>().unwrap() + 2);
}
