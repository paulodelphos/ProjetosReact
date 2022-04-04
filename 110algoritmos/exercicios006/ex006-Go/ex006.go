package main

import (
	"fmt"
	"math"
)

func main() {

	var num float64

	fmt.Printf("Digite um número: ")
	fmt.Scan(&num)

	dobro := num * 2
	triplo := num * 3
	raiz := math.Pow(num, 0.5)

	fmt.Printf("O dobro de %v é %v \nO tripo é: %v. \nA raiz é: %.2f", num, dobro, triplo, raiz)

}
