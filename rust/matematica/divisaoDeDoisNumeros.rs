use std::io; //biblioteca de input
//fn main é a função que roda sempre que o código começa
fn main() {

    println!("Write 2 numbers (press enter between them): ");

    let mut num = String::new(); //faz com que a variável seja em formato de input
    let mut ndois = String::new();

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("first number is: {}", num);  

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut ndois)
        .expect("Failed to read line");
    println!("second number is: {}", ndois);

    let num: i32 = num.trim().parse().expect("Please type a number!"); //.transforma em int o string
    let ndois: i32 = ndois.trim().parse().expect("Please type a number!");

    if ndois == 0{
        println!("Não se pode dividir por zero! Tente novamente");
        main()
    } else{
        let resultado = ndois / num;
        println!("The result is: {} + {} = {}", num,ndois,resultado) //para se ter variável em um texto se declara deste jeito
    }


}

  
    
