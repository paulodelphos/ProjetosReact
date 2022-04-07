package main

import "fmt"

func main() {
	var c float64
	var f float64

	fmt.Println("Informe a temperatura em Celsius: ")
	fmt.Scanln(&c)
	f = (9*c)/5 + 32
	fmt.Printf("\nA temperatura em %v C corresponde em %v F.", c, f)

}
