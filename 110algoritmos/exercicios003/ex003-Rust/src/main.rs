use std::io;

fn main() {
    let mut n1:String = String::from("");
    let mut n2:String = String::from("");

    println!("Digite o primeiro valor: ");
    io::stdin()
        .read_line(&mut n1)
        .expect("Failed to read line");
    let n1:i32 = n1.trim().parse().expect("falha na conversao de valor");

    println!("Digite o segundo valor: ");
    io::stdin()
        .read_line(&mut n2)
        .expect("falha na leitura");
    let n2:i32 = n2.trim().parse().expect("Falha naconversao de valor n2");

    let total:i32 = n1 + n2;

       
    println!("A soma dos valores {} e {} = {}", n1, n2, total);
}
