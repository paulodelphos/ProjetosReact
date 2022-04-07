use std::io;

fn main() {
    
    let mut angulo:String = String::from("");
   // let mut seno:f64;
    //let mut coseno:f64;
    //let mut tangente:f64;
    
    println!("Digite o valor do angulo: ");
    io::stdin()
        .read_line(&mut angulo)
        .expect("Error na leitura do angulo");
    let angulo:f64 = angulo.trim().parse().expect("Error conversao angulo");

    let seno = ((angulo * 3.1415)/180.0).sin();
    let coseno = ((angulo * 3.1415)/180.0).cos();
    let tangente = ((angulo * 3.1415)/180.0).tan();

    println!("O SENO de {} é igual a {:.2}", angulo, seno);
    println!("O COSENO de {} é igual a {:.2}", angulo, coseno);
    println!("O TANGENTE de {} é igual a {:.2}", angulo, tangente);
    
}