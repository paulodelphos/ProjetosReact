use std::io;

fn main() {

    let mut n1:String = String::from("");
    let mut n2:String = String::from("");

    println!("Digite a primeira nota: ");
    io::stdin()
        .read_line(&mut n1)
        .expect("Erro ao ler primeira nota");
    let n1:f32 = n1.trim().parse().expect("Erro ao converter n1");

    println!("Digite a segunda nota: ");
    io::stdin()
        .read_line(&mut n2)
        .expect("Erro ao ler segunda nota");
    let n2:f32 = n2.trim().parse().expect("Erro ao converter n2");

    let media = (n1 + n2)/2.0;

    println!("A média entre {} e {} é {:.2}", n1, n2, media);


}
