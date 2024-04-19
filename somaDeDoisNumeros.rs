use std::io;

fn main(){
    let mut num = String::new();
    println!("Escreva um número");

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    
    let num: i32 = num.trim().parse().expect("Please type a number!");

    
    let mut ndois = String::new();
    println!("Escreva um número");
    
     io::stdin()
        .read_line(&mut ndois)
        .expect("Failed to read line");
    
    let ndois: i32 = ndois.trim().parse().expect("Please type a number!");
   
    
    println!("{}, {}", num, ndois);

    let resultado = num + ndois;

    println!("{}", resultado)
    
}

