fn main() {
    let mut s = String::from("Hello");
    let s1 = &s[1..2];
    let s2 = "string";
    // let i = first_word(&s);
    println!("{}", s1)
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();// 将String转换字节数组
//     for(i, &item) in bytes.iter().enumerate() {
//        if item == b'' {
//            return i;
//        }
//     }
//     s.len()
// }
