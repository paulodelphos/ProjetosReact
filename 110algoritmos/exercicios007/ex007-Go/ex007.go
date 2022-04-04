package main

import "fmt"

func main() {
	var n1 float32
	var n2 float32

	fmt.Println("Digite a primeira nota: ")
	fmt.Scan(&n1)

	fmt.Println("Digite a segunda nota: ")
	fmt.Scan(&n2)

	media := (n1 + n2) / 2

	fmt.Printf("A média entre %v e %v é %.1f", n1, n2, media)

}
