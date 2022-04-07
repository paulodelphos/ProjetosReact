let dias = Number(prompt("Quantos dias alugados? "));
let km = Number(prompt("Quantos Km rodados? "));
let total = (dias*60) + (km * 0.15);
console.log(`O total a pagar pelo aluguel é R$ ${total}`);