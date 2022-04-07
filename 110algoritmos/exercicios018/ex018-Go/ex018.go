package main

import (
	"fmt"
	"math"
)

func main() {
	var angulo float64

	fmt.Println("Digite o valor do angulo: ")
	fmt.Scanln(&angulo)

	seno := math.Sin((angulo * math.Pi) / 180.0)
	conseno := math.Cos((angulo * math.Pi) / 180.0)
	tangente := math.Tan((angulo * math.Pi) / 180.0)

	fmt.Printf("\nO SENO de %v é: %.2f", angulo, seno)
	fmt.Printf("\nO COSENO de %v é: %.2f", angulo, conseno)
	fmt.Printf("\nA TANGENTE de %v é: %.2f", angulo, tangente)

}
