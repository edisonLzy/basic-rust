fn main() {}

fn show_str() -> &String {
    let s = String::from("Hello");
    &s // &s在函数结束之后指向了已经被销毁的内存
}
