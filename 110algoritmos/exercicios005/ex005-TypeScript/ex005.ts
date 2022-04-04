//  Faça um programa qu eleia um número inteiro 
//  e mostre na tela seu suecessor e seu antecessor
let num:number, sucessor:number, antecessor:number;
num = Number(prompt("Digite um valor inteiro: "));
sucessor = num + 1;
antecessor = num -1;

console.log(`O antecessor de ${num} é: ${antecessor} e seu sucessor é: ${sucessor}`)
