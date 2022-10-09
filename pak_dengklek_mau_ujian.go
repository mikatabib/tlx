package main

import "fmt"

func main() {
	var num int
	fmt.Scanf("%d", &num)
	if num < 0 || num > 100 {
		fmt.Println("TIDAK")
	} else {
		fmt.Println("YA")
	}
}
