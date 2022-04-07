package main

import (
	"fmt"
	"math"
)

func main() {

	var co float64
	var ca float64

	fmt.Println("Comprimento do cateto oposto: ")
	fmt.Scan(&co)

	fmt.Println("Comprimento do cateto adjacente: ")
	fmt.Scan(&ca)

	hipo := math.Hypot(co, ca)

	fmt.Printf("A hipotenusa vai medir %.2f", hipo)

}
