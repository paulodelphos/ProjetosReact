const readlineSync = require('readline-sync');

let n1 = Number(readlineSync.question('Digite o primeiro valor: '));
let n2 = Number(readlineSync.question("Digite o segundo valor: "));
const total = n1 + n2;
console.log(`A soma de ${n1} + ${n2} = ${total}`);