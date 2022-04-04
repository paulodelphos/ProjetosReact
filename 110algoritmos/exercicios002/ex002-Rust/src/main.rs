use std::io;

fn main() {
    let mut nome = String::from("");

    println!("Digite o seu nome: ");
    io::stdin()
        .read_line(&mut nome)
        .expect("Failed to read line");

    println!("É um prazer te conhecer {nome}!")
}