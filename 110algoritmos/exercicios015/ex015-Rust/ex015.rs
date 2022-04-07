use std::io;

fn main(){
    let mut dias:String = String::from("");
    let mut km:String = String::from("");

    println!("Quantos dias alugados? ");
    io::stdin()
        .read_line(&mut dias)
        .expect("Erro ao ler dias");
    let dias:f32 = dias.trim().parse().expect("Error: conversao dias");

    println!("Quantos Km rodados? ");
    io::stdin()
        .read_line(&mut km)
        .expect("Error ao ler km");
    let km:f32 = km.trim().parse().expect("error: conversao km");

    let total = (dias * 60.0) + (km * 0.15);

    println!("O total a pagar é R$ {}", total);


}