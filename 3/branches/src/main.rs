fn main() {
    let number = 3;

    let str = if number < 5 {
        println!("condition is true");
        "hi".to_string()
    } else {
        println!("condition is false");
        "bye".to_string()
    };
    println!("{}", str);
}
