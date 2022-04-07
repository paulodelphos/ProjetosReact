let sal = Number(prompt("Qual é o salário do Funcionário? R$"));
const aumento = sal * 1.15;
console.log(`Um funcionáro que ganhava R$${sal}, com 15% de aumento, passa a receber R$${aumento.toFixed(2)}.`)