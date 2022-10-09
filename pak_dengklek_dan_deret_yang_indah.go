package main

import "fmt"

func main() {
	var s, n, d int

	fmt.Scanf("%d %d %d", &s, &n, &d)

	for it := 0; it < n; it++ {
		fmt.Println(s)
		s += d
	}
}
