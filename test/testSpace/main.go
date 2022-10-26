package main

import "fmt"

func main() {
	a := 1
	b := func() {
		fmt.Println(a)
	}
	a = 2
	b()
}
