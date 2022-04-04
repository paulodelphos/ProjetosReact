use std::io;

fn main() {

    let mut num:String = String::from("");
    //let mut raiz:f32;

    println!("Digite um número: ");
    io::stdin()
        .read_line(&mut num)
        .expect("Falha ao ler numero");
    let num:f32 = num.trim().parse().expect("Erro na conversao");
    let dobro = num * 2.0;
    let triplo = num * 3.0;
    let raiz = f32::powf(num,0.5);

    println!("O dobro de {} é {}.\nO triplo é {}.\nA raiz é {:.2}.", num, dobro, triplo, raiz);
}
