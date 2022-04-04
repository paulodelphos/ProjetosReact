use std::io;

fn main() {

    let mut metros:String = String::from("");

    println!("Digite uma distancia em metros");
    io::stdin()
        .read_line(&mut metros)
        .expect("Fala ao ler o metros");
    let metros:f32 = metros.trim().parse().expect("Erro na conversao");
    let centi = metros * 100.0;
    let mili = metros * 1000.0;

    println!("{} metros são {} centimetros e {} milimetros", metros, centi, mili);






}
