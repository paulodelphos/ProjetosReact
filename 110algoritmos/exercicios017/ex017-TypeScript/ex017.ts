let co = Number(prompt("Comprimento do cateto oposto: "));
let ca = Number(prompt("Comprimento do cateto adjacente: "));
let hipo = Math.hypot(co, ca);
console.log(`A hipotenusa vai medir ${hipo.toFixed(2)}`);