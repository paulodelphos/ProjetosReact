package main

import (
	"fmt"
)

var n1, n2, total int

func main() {
	fmt.Println("Digite um valor: ")
	fmt.Scanln(&n1)
	//fmt.Println(reflect.TypeOf(n1))

	fmt.Printf("Digite outro valor: ")
	fmt.Scanln(&n2)

	total = n1 + n2

	fmt.Printf("A soma de %v + %v = %v", n1, n2, total)
}
