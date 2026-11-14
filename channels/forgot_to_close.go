package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Println("# too_many_sends()")
	too_many_sends()
	time.Sleep(100 * time.Millisecond)

	fmt.Println()
	fmt.Println("# too_many_receives()")
	too_many_receives()
	time.Sleep(100 * time.Millisecond)
}

const size = 0

func too_many_sends() {
	c := make(chan int, size)
	go func() {
		c <- 1
		c <- 2
		c <- 3
		c <- 4
		fmt.Println("finished sending") // never printed
	}()

	fmt.Println("val: ", <-c)
	fmt.Println("val: ", <-c)
	fmt.Println("val: ", <-c)
}

func too_many_receives() {
	c := make(chan int, size)
	go func() {
		c <- 1
		c <- 2
		c <- 3
		fmt.Println("finished sending")
	}()

	fmt.Println("val: ", <-c)
	fmt.Println("val: ", <-c)
	fmt.Println("val: ", <-c)
	fmt.Println("val: ", <-c) // deadlock error
}
