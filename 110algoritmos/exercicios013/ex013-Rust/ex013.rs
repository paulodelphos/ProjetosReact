use std::io;

fn main() {
    
    let mut salario:String = String::from("");

    println!("Qual é o salário do funcionario? R$");
    io::stdin()
        .read_line(&mut salario)
        .expect("Error ao ler salario");
    let salario:f32 = salario.trim().parse().expect("Erro ao converter salario");

    let aumento = salario * 1.15;

    println!("Um funcionario que ganhava R${}, com 15% de aumento, passa a receber R${:.2}.", salario, aumento);


}