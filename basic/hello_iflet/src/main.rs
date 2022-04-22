fn main() {
    let some_number = Some(3);
    if let Some(state) = some_number {
        println!("some_number is {}", state)
    } else {
        println!("some_number is other")
    }
    // match some_number {
    //     Some(3) => println!("some_number is 3"),
    //     _ => println!("some_number is other"),
    // }
}
