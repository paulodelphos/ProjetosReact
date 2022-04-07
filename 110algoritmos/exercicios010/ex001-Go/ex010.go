package main

import "fmt"

func main() {
	var real float32

	fmt.Print("Digite quantos Reais você tem na cardeira: R$")
	fmt.Scan(&real)

	dollar := real / 4.61

	fmt.Printf("\nCom R$ %.2f reais você pode comprar $ %.2f dollares\n", real, dollar)
}
