use std::io;

fn main() {

    let mut largura:String = String::from("");
    let mut altura:String = String::from("");

    println!("Digite a largura da parede: ");
    io::stdin()
        .read_line(&mut largura)
        .expect("Erro ao ler a largura");
    let largura:f32 = largura.trim().parse().expect("Erro conversao largura");

    println!("\nDigite a altura da parede: ");
    io::stdin()
        .read_line(&mut altura)
        .expect("Erro leitura de altura");
    let altura:f32 = altura.trim().parse().expect("Erro conversao altura");

    let area = altura * largura;
    let tinta = area/2.0;

    println!("Sua parede tem a dimensao de {}x{}. E sua área e de {}m.", altura, largura, area);
    println!("Para pintar a parede precisará de {}l de tinta", tinta);


}