package main

import "fmt"

func main() {
	a := make(map[int]bool)
	a[1] = true
	a[3] = true
	a[5] = true
	a[10] = true
	for i, _ := range a {
		fmt.Println(i)
		delete(a, i)
	}
	fmt.Println(a)
}
