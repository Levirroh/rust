use std::io;
fn main() {

    println!("Write 2 numbers (press enter between them): ");

    let mut num = String::new();
    let mut ndois = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("first number is: {}", num);

    io::stdin()
        .read_line(&mut ndois)
        .expect("Failed to read line");
    println!("second number is: {}", ndois);

    let num: i32 = num.trim().parse().expect("Please type a number!");
    let ndois: i32 = ndois.trim().parse().expect("Please type a number!");

    let resultado = ndois + num;
    
    println!("The result is: {} + {} = {}", num,ndois,resultado)
}
