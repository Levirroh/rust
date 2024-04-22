
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
    

    match escolha:
        "1" => soma();
        "2" => subtracao();
        "3" => divisao();
        "4" => multiplicar();
        "5" => fatorial();
        "6" => exponencial();
}


fn soma(){
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

    let resultado = ndois + num;
    
    println!("The result is: {} + {} = {}", num,ndois,resultado) //para se ter variável em um texto se declara deste jeito
}



fn subtracao(){
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

    let resultado = num - ndois;
    
    println!("The result is: {} - {} = {}", num,ndois,resultado) //para se ter variável em um texto se declara deste jeito
}


fn multiplicar(){
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

    let resultado = ndois * num;
    
    println!("The result is: {} x {} = {}", num,ndois,resultado) //para se ter variável em um texto se declara deste jeito
}

fn divisao(){
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

    let num: f64 = num.trim().parse().expect("Please type a number!"); //.transforma a String em float (f64)
    let ndois: f64 = ndois.trim().parse().expect("Please type a number!");

    if ndois == 0.0{
        println!("Não se pode dividir por zero! Tente novamente");
        main()
    } else{
        let resultado = num / ndois as f64; // deixa o resultado em float
        println!("The result is: {} / {} = {}", num,ndois,resultado) //para se ter variável em um texto se declara deste jeito
    }
}
