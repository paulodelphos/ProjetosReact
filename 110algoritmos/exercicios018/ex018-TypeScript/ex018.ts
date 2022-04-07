let angulo = Number(prompt("Digite um angulo: "))
let seno = Math.sin((angulo * Math.PI)/180.0)
let coseno = Math.cos((angulo * Math.PI)/180.0)
let tangente = Math.tan((angulo * Math.PI)/180.0)

console.log(`O SENO de ${angulo} é ${seno.toFixed(2)}`)
console.log(`O CONSENO de ${angulo} é ${coseno.toFixed(2)}`)
console.log(`O TANGENTE de ${angulo} é ${tangente.toFixed(2)}`)