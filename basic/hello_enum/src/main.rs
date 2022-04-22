fn main() {
    let some_number = Some(5);
    let some_str: Option<String> = Some(String::from("haha"));
    match &some_number {
        None => println!("the value is not exist"),
        Some(state) => println!("{}", state),
    }
    println!("the number is {:?}", some_str)
}
