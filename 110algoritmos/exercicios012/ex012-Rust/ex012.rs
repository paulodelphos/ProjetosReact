use std::io;

fn main() {

    let mut valor:String = String::from("");

    println!("Qual é o preço do produto? R$");
    io::stdin()
        .read_line(&mut valor)
        .expect("Erro ao ler valor");
    let valor:f32 = valor.trim().parse().expect("Erro ao converter valor");

    let descontado = valor * 0.95;

    println!("O produto que custava R${:.2}, na promoção com desconto de 5% vai custar R${:.2}", valor, descontado);

}