package main

import "fmt"

func main() {
	var largura, altura float32

	fmt.Print("Digite a largura da parede: ")
	fmt.Scan(&largura)
	fmt.Print("\nDigite a altura da parede: ")
	fmt.Scan(&altura)

	area := largura * altura
	tinta := area / 2

	fmt.Printf("\nSua parede tem dimensao de %vx%v e sua area eh de %vm", largura, altura, area)
	fmt.Printf("\nPara pintar essa parede eh necessario %.2f litros de tinta.", tinta)
}
