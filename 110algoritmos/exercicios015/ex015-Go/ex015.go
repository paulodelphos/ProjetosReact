package main

import "fmt"

func main() {
	var dias float32
	var km float32

	fmt.Println("Quandos dias alugados? ")
	fmt.Scan(&dias)
	fmt.Println("Quantos Km rodados? ")
	fmt.Scan(&km)

	total := (dias * 60) + (km * 0.15)

	fmt.Printf("\nO total a pagar é R$ %.2f", total)

}
