fn main() {
    let five = Some(5);
    println!("{:?}", plus_one(five));
    println!("{:?}", plus_one(None))
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None, // 匹配 除了 Some以为的情况
    }
}
