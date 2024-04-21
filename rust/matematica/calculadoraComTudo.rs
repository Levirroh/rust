use std::io; //biblioteca de input
//fn main é a função que roda sempre que o código começa
fn main() {
    calcular();
}
fn calcular(){
    
    println!("Escreva o primeiro número!");

    let mut num = String::new(); //faz com que a variável seja em formato de input
    let mut ndois = String::new();

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("O primeiro número é: {}", num);  
    
    println!("Escreva o segundo número!");

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut ndois)
        .expect("Failed to read line");
    println!("O segundo número é: {}", num);  

    let num: i64 = num.trim().parse().expect("Please type a number!"); //.transforma em int o string
    let ndois: i64 = ndois.trim().parse().expect("Please type a number!");
    
    println!("Escolha a operação:");
    println!("1: Soma de Dois Números:");
    println!("2: Subtração de Dois Números");
    println!("3: Divisão de Dois Números");
    println!("4: Multiplicação de Dois Números");
    let mut escolha = String::new();
    
    io::stdin() //isso faz com que o código leia um input
            .read_line(&mut escolha)
            .expect("Failed to read line");
    
    let mut resultado = 0;
    let escolha: i32 = escolha.trim().parse().expect("Please type a number!"); //.transforma em int o string
    
    match escolha {
        1 => resultado = num + ndois,
        2 => resultado = num - ndois,
        3 => resultado = num / ndois,
        4 => resultado = num * ndois,
        _ => println!("Valor Inválido"), 
    }
    
    println!("{}", resultado);
    
    println!("Deseja fazer outra conta? sim = 1, nao = 0");
    let mut denovo = String::new();
    
    io::stdin() //isso faz com que o código leia um input
            .read_line(&mut denovo)
            .expect("Failed to read line");
    let denovo: i32 = denovo.trim().parse().expect("Please type a number!"); //.transforma em int o string
    
    if denovo == 1{
        main();
    }
    println!("Tchau! Até a próxima!")
}
