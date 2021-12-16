package main

import (
	"bufio"
	"fmt"
	"os"
)

func errCheck(e error) {
	if e != nil {
		panic(e)
	}
}

func run(fname string) {
	file, err := os.Open(fname)
	errCheck(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}

	errCheck(scanner.Err())
}

func main() {
	run("test-input.txt")
	run("input.txt")
}
