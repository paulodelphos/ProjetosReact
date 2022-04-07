package main

import "fmt"

func main() {

	var valor float32

	fmt.Print("Qual é o preço do produto? R$")
	fmt.Scan(&valor)

	descontado := valor * 0.95

	fmt.Printf("\nO produto que custava R$%.2f, na promoção com desconto de 5%% porcento, vai custar R$%.2f", valor, descontado)

}
