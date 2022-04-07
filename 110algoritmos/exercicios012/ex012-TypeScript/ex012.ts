let valor = Number(prompt("Qual é preço do produto? R$"));
let descontado = valor * 0.95;
console.log(`O produto que custava R$${valor}, na promoção com desconto de 5% vai custar R$${descontado.toFixed(2)}.`)