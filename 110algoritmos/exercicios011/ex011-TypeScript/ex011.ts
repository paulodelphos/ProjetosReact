let largura = Number(prompt("Digite a largura da parede:"));
let altura = Number(prompt("Digite a altura da parede:"));
const area = altura * largura;
const tinta = area/2
console.log(`Sua parede tem a dimensao de ${largura}x${altura} e sua area eh de ${area}m.`);
console.log(`Para pintar essa parede, vc precisara de ${tinta} de tinta`);
