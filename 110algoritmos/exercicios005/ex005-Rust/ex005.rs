//faça um programa qu eleia um número inteiro e mostre na tela seu suecessor e seu
//antecessor
use std::io;

fn main() {

    
    let mut num:String = String::from("");
    //let antecessor:i32;
   // let sucessor:i32;
    
    println!("Digite um valor inteiro: ");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num:i32 = num.trim().parse().expect("Erro na conversao");
    let antecessor = num -1;
    let sucessor = num + 1;

    println!("O antecessor de {} é {} e seu sucessor é {}",num, antecessor, sucessor);
}