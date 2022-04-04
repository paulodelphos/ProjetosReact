package main

import "fmt"

func main() {
	var valor int
	fmt.Println("Digite um valor: ")
	fmt.Scan(&valor)
	fmt.Printf("Valor: %v, Tipo: %T", valor, valor)

}
