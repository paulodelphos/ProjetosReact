const readlineSync = require('readline-sync')
//onst read = PromptFunction();


let nome = readlineSync.question("Digite o seu nome? ")
console.log(`Muito prazer em te conhecer ${nome}`);