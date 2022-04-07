package main

import "fmt"

func main() {
	var sal float32

	fmt.Print("Qual é o salário do funcionário? R$")
	fmt.Scan(&sal)
	aumento := sal * 1.15
	fmt.Printf("\nUm funcionário que ganhava R$%v, com 15%% de aumento, passa a ganhar R$%.2f", sal, aumento)

}
