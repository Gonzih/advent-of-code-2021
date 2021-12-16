package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func errCheck(e error) {
	if e != nil {
		panic(e)
	}
}

func bin2dig(s string) int {
	d, err := strconv.ParseInt(s, 2, 64)
	errCheck(err)
	return int(d)
}

var vocab = map[string]string{
	"0": "0000",
	"1": "0001",
	"2": "0010",
	"3": "0011",
	"4": "0100",
	"5": "0101",
	"6": "0110",
	"7": "0111",
	"8": "1000",
	"9": "1001",
	"A": "1010",
	"B": "1011",
	"C": "1100",
	"D": "1101",
	"E": "1110",
	"F": "1111",
}

func decode(ln string) string {
	decoded := ""

	for _, c := range ln {
		s := string(c)
		fmt.Print(s)
		decoded += vocab[s]
	}

	return decoded
}

type packet struct {
	line string
}

func (p *packet) version() int {
	return bin2dig(p.line[0:3])
}

func (p *packet) id() int {
	return bin2dig(p.line[3:6])
}

func (p *packet) ltid() int {
	return bin2dig(p.line[7:8])
}

func (p *packet) len() int {
	return bin2dig(p.line[8:22])
}

func (p *packet) val() int {
	s := ""
	fmt.Println()
	if p.id() == 4 {
		s += p.line[7:11]
		fmt.Println(s)
		s += p.line[12:16]
		fmt.Println(s)
		s += p.line[17:21]
	}
	fmt.Println(s)
	return bin2dig(s)
}

func run(fname string) {
	file, err := os.Open(fname)
	errCheck(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	ln := ""

	for scanner.Scan() {
		ln = scanner.Text()
	}

	pck := packet{line: decode(ln)}

	fmt.Println()
	fmt.Println(pck)
	fmt.Println(pck.version())
	fmt.Println(pck.id())

	errCheck(scanner.Err())
}

func main() {
	run("test-input.txt")
	// run("input.txt")
}
