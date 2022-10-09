package main

import (
	"fmt"
	"math"
)

func reverse(n int) int {
	digit := int(math.Log10(float64(n))) + 1
	var rev int
	for it := digit - 1; it >= 0; it-- {
		rev += int(float64(n%10) * math.Pow(10.0, float64(it)))
		n /= 10
	}
	return rev
}

func main() {
	var n int
	fmt.Scanf("%d", &n)

	nums := make([]int, n)
	for it := 0; it < n; it++ {
		fmt.Scanf("%d", &nums[it])
	}

	for it := 0; it < n; it++ {
		fmt.Println(reverse(nums[it]))
	}
}
