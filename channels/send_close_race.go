package main

import (
	"fmt"
	"time"
)

func fibonacci(n int, c chan int) {
	x, y := 0, 1
	for i := 0; i < n; i++ {
		c <- x
		x, y = y, x+y
	}
	//fmt.Println("fib received: ", <-c)
	close(c)
	//fmt.Println("fib received: ", <-c)
	fmt.Println("exit fibonacci")
}

func asdf(n int, c chan int) {
	<-time.After(2000 * time.Millisecond)
	c <- n
}

func main() {
	c := make(chan int)
	go fibonacci(10, c)
	go asdf(12345, c)
	tick := time.Tick(100 * time.Millisecond)
	<-tick

	for i := range c {
		<-tick
		fmt.Println("main received: ", i)
	}

	<-time.After(2000 * time.Millisecond)

	fmt.Println("exit main")
}
