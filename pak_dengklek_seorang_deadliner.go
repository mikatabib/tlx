package main

import "fmt"

func main() {
	var time int
	fmt.Scanf("%d", &time)

	fmt.Println(time / 3600)
	fmt.Println(time / 60 % 60)
	fmt.Println(time % 60)
}
