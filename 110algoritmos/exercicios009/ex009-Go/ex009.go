// 					TABUADA
package main

import "fmt"

func main() {

	var num int

	fmt.Print("\nDigite um valor para a tabuada: ")
	fmt.Scan(&num)

	fmt.Println("=-=-=-=-=-=")
	for i := 1; i <= 10; i++ {
		fmt.Printf("%v x %v = %v\n", num, i, (num * i))
	}
	fmt.Println("=-=-=-=-=-=")
}
