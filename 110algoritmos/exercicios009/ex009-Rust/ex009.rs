use std::io;

fn main() {

    let mut num:String = String::from("");

    println!("Digite um valor para tabuada: ");
    io::stdin()
    .read_line(&mut num)
    .expect("Falha ao ler o numero");
    let num:i32 = num.trim().parse().expect("Erro na conversao de num");
    
    println!("=-=-=-=-=-=");
    for n in 1..11 {
        println!("{} x {} = {}", num, n, (num*n));
    }
    println!("=-=-=-=-=-=");
}