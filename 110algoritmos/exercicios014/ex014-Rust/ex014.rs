use std::io;

fn main() {
    let mut c:String = String::from("");
    let f:f32;

    println!("Informe a temperatura em Celsius");
    io::stdin()
        .read_line(&mut c)
        .expect("Erro na leitura de C");
    let c:f32 = c.trim().parse().expect("Erro na conversao de C");

    f = (9.0 * c ) / 5.0 + 32.0;

    println!("A temperatura em {} C corresponde {} F", c, f);
}