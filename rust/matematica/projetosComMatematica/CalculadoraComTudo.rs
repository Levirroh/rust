use std::io;


fn main() {
    let mut escolha = String::new();
    println!("Escolha uma operação:");
    println!("[1] Soma.");
    println!("[2] Subtração.");
    println!("[3] Divisão.");
    println!("[4] Multiplicação.");
    println!("[5] Fatorial.");
    println!("[6] Exponenciais.");
    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut escolha)
        .expect("Failed to read line");
    
    let escolha: i32 = escolha.trim().parse().expect("Please type a number!"); //.transforma em int o string

    match escolha {
        1 => soma(),
        2 => subtracao(),
        3 => divisao(),
        4 => multiplicar(),
        5 => fatorial(),
        6 => exponencial(),
        _ => println!("Escolha inválida, reinicie o código.")
    }
}


fn soma(){
     println!("Escreva o primeiro número. ");

    let mut num = String::new(); //faz com que a variável seja em formato de input
    let mut ndois = String::new();

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("O primeiro número é: {}", num);  
     println!("Escreva o segundo número. ");

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut ndois)
        .expect("Failed to read line");
    println!("O segundo número é: {}", ndois);

    let num: i32 = num.trim().parse().expect("Please type a number!"); //.transforma em int o string
    let ndois: i32 = ndois.trim().parse().expect("Please type a number!");

    let resultado = ndois + num;
    
    println!("O resultado é: {} + {} = {}", num,ndois,resultado) //para se ter variável em um texto se declara deste jeito
}



fn subtracao(){
    println!("Escreva o primeiro número");

    let mut num = String::new(); //faz com que a variável seja em formato de input
    let mut ndois = String::new();

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("O primeiro número é: {}", num);  
    println!("Escreva o segundo número");

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut ndois)
        .expect("Failed to read line");
    println!("O segundo número é: {}", ndois);

    let num: i32 = num.trim().parse().expect("Please type a number!"); //.transforma em int o string
    let ndois: i32 = ndois.trim().parse().expect("Please type a number!");

    let resultado = num - ndois;
    
    println!("O resultado é: {} - {} = {}", num,ndois,resultado) //para se ter variável em um texto se declara deste jeito
}


fn multiplicar(){
    println!("Escreva o primeiro número ");

    let mut num = String::new(); //faz com que a variável seja em formato de input
    let mut ndois = String::new();

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("O primeiro número é: {}", num);  
    println!("Escreva o segundo número ");

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut ndois)
        .expect("Failed to read line");
    println!("O segundo número é: {}", ndois);

    let num: i32 = num.trim().parse().expect("Please type a number!"); //.transforma em int o string
    let ndois: i32 = ndois.trim().parse().expect("Please type a number!");

    let resultado = ndois * num;
    
    println!("O resultado é: {} x {} = {}", num,ndois,resultado) //para se ter variável em um texto se declara deste jeito
}

fn divisao(){
    println!("Escreva o primeiro número.");

    let mut num = String::new(); //faz com que a variável seja em formato de input
    let mut ndois = String::new();

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("O segundo número é: {}", num);  

    println!("Escreva o segundo número");

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut ndois)
        .expect("Failed to read line");
    println!("O segundo número é: {}", ndois);

    let num: f64 = num.trim().parse().expect("Please type a number!"); //.transforma a String em float (f64)
    let ndois: f64 = ndois.trim().parse().expect("Please type a number!");

    if ndois == 0.0{
        println!("Não se pode dividir por zero! Tente novamente");
        main()
    } else{
        let resultado = num / ndois as f64; // deixa o resultado em float
        println!("O resultado é: {} / {} = {}", num,ndois,resultado) //para se ter variável em um texto se declara deste jeito
    }
}


fn exponencial(){
      println!("Escreva o número da base: ");
    
    let mut num = String::new(); //faz com que a variável seja em formato de input
    let mut ndois = String::new();

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("A base é: {}", num);  

    println!("Escreva o número do expoente: ");


    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut ndois)
        .expect("Failed to read line");
    println!("O expoente é: {}", ndois);

    let num: i64 = num.trim().parse().expect("Please type a number!"); //.transforma a String em int (i64 pq pode ser grande))
    let ndois: i64 = ndois.trim().parse().expect("Please type a number!");
    
    let mut resultado = 1 as i64;
    for i in 0..ndois {
        resultado = resultado * num;
    } 
    println!("{} elevado a {} é igual a: {}",num,ndois,resultado);
}

fn fatorial(){
    println!("Escreva o número que seja fatoriado: ");
    
    let mut num = String::new(); //faz com que a variável seja em formato de input

    io::stdin() //isso faz com que o código leia um input
        .read_line(&mut num)
        .expect("Failed to read line");
    println!("O número é: {}", num);  

    let num: i64 = num.trim().parse().expect("Please type a number!"); //.transforma a String em float (f64)
    let mut resto = num;
    let mut resultado = 1 as i64;
    for i in 1..num {
        if i == 1 {
            resultado = resultado * (num);
        }
            resto = resto - 1;
            resultado = resultado * (resto);        
    } 
    println!("O resultado de {}! é igual a: {}",num,resultado);
}
