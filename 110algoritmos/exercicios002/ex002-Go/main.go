package main

import "fmt"

var nome string = "Delphos"

func main() {

	println("Digite o seu nome")
	fmt.Scanln(&nome)
	println("É um prazer te conhecer " + nome + "!")
}
