const readlineSync = require('readline-sync');

let valor = readlineSync.question("Digite algo: ");
console.log(`O tipo primitivo desse valor é ${typeof(valor)}`)
console.log(`Só tem espaços? ${valor.indexOf(" ")}`)
