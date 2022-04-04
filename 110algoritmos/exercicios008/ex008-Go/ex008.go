package main

import "fmt"

func main() {
	var metros float32

	fmt.Print("Digite uma distancia em metros: ")
	fmt.Scan(&metros)

	cent := metros * 100
	mili := metros * 1000

	fmt.Printf("%v metros sao %v centimetros e %v milimetros.", metros, cent, mili)

}
