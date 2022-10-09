package main

import "fmt"

func gcd(n, m int) int {
	for m != 0 {
		if m < n {
			m, n = n, m
		}
		m = m % n
	}
	return n
}

func main() {
	var n int
	fmt.Scanf("%d", &n)

	var merchants = make([]int, n)

	for it := 0; it < n; it++ {
		fmt.Scanf("%d", &merchants[it])
	}

	comDiv := merchants[0]
	for it := 1; it < n; it++ {
		num := comDiv * merchants[it]
		comDiv = num / gcd(comDiv, merchants[it])
	}
	fmt.Println(comDiv)
}
