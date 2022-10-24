package main

import "fmt"

func main() {
	a := []int{1, 2, 3, 4}
	for i := 0; i < len(a); i++ {
		if i%2 == 0 {
			a = append(a, i)
		}
		fmt.Println(a[i])
	}
	fmt.Println(a)
}
