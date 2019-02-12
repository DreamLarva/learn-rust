pub fn main() {
    let mut x = 5;
    println!("the value of is: {}", x);
    // 使用可变的变量 重新赋值的时候 不能改变原变量的类型
    x = 6;
    println!("The value of is: {}", x);

    let y = 1;
    let y = y + 1;
    let y = y * 2;
    println!("The value of is: {}", y);

    // 使用shadow 使用新的变量 隐藏原变量 可以无视 变量的类型
    let spaces = " ";
    let spaces = spaces.len();
    println!("The space of is: {}", spaces)
}
