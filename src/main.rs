use std::io;

#[derive(Debug)] // 添加注解 用于打印结构体实例
struct Rectangle {
    width: i32,
    height: i32,
    filled: bool,
}
fn main() {
    let mut rec = Rectangle {
        width: 0,
        height: 0,
        filled: true,
    };
    let width = get_input("please input width");
    let height = get_input("please input height");
    rec = Rectangle {
        width,
        height,
        ..rec // 这里并不会覆盖已经显示给定的字段值
    };
    let result = area(&rec);
    println!("{:#?}", rec); // 换行打印
    println!("{:?}", rec); // 不换行打印
    println!("{} * {} = {}", width, height, result);
}

fn get_input(tip: &str) -> i32 {
    println!("{}", tip);
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("fail to read the line ");
    let value: i32 = value.trim().parse().expect("parse fail");
    return value;
}
fn area(rec: &Rectangle) -> i32 {
    rec.height * rec.width
}
