fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    let s2 = &s;
    println!("{} {}", s1, s2);
}
