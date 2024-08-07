fn main() {
    // println! 是打印宏，并不是函数
    println!("Hello, world!");
    println!("please input your guess");

    // let mut guess
    // let guess
    // rust 变量默认是不可变的
    // 加入 mutability 可变性的意思表示变量可变
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");
    println!("You guessed: {}", guess);
}
