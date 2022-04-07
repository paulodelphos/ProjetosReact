use std::io;

fn main() {
    let mut real:String = String::from("");

    println!("Digite quantos reais tem na carteira: R$ ");
    io::stdin()
        .read_line(&mut real)
        .expect("Erro ao ler real");
    let real:f32 = real.trim().parse().expect("Erro ao converter real");

    let dollar = real/4.61;

    println!("Com R$ {} reais você pode comprar $ {} dollares", real, dollar);

}