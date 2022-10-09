package main

import (
	"fmt"
	"math"
)

func trailingZeroes(n int) int {
	k := int(math.Log(5.0) * float64(n))
	zeroes := 0
	p := 5
	for it := 0; it < k; it++ {
		cur := n / p
		if cur == 0 {
			break
		}
		zeroes += cur
		p *= 5
	}
	return zeroes
}

func main() {
	var n int
	fmt.Scanf("%d", &n)

	fmt.Println(trailingZeroes(n))
}
