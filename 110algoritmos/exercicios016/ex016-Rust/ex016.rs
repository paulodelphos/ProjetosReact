use std::io;

fn main() {
    let mut num:String = String::from("");

    println!("Digite um numero fracionado");
    io::stdin()
        .read_line(&mut num)
        .expect("error leitura");
    let num:f64 = num.trim().parse().expect("erro conversao");

    let truncado = num.trunc();

    println!("O numero {} truncado é {}", num, truncado);
}