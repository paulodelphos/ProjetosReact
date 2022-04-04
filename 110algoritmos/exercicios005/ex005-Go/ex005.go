package main

import "fmt"

func main() {
	var num int

	fmt.Println("Digite um numero inteiro: ")
	fmt.Scanln(&num)
	sucessor := num + 1
	antecessor := num - 1

	fmt.Printf("O antecessor de %v eh %v e seu sucessor eh %v", antecessor, num, sucessor)
}
