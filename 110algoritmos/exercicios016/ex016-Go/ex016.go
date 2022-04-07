package main

import (
	"fmt"
	"math"
)

func main() {
	var num float64

	fmt.Println("Digite um numero fracionario: ")
	fmt.Scanln(&num)

	truncado := math.Trunc(num)

	fmt.Printf("\nO numero %v truncado é %v", num, truncado)

}
