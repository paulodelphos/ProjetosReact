let real = Number(prompt("Digite quantos Reais você tem na carteira: R$"));
let dollar = real/4.61;
console.log(`Com R$${real.toFixed(2)} você pode comprar $${dollar.toFixed(2)} dollares`);