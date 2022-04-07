use std::io;

fn main(){
    let mut ca:String = String::from("");
    let mut co:String = String::from("");

    println!("Digite o cateto adjacente: ");
    io::stdin()
        .read_line(&mut ca)
        .expect("Erro na leitura ca");
    let ca:f64 = ca.trim().parse().expect("Erro conversao ca");

    println!("Digite o cateto oposto: ");
    io::stdin()
        .read_line(&mut co)
        .expect("Error leitura co");
    let co:f64 = co.trim().parse().expect("Error conversao co");

    let hipo = ca.hypot(co);

    println!("A hipotenusa é igual a: {:.2}", hipo);

}