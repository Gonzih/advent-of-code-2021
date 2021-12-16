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
		decoded += vocab[s]
	}

	return decoded
}

type packet struct {
	line string
}

func (p *packet) read_rest_str(start int) string {
	return p.line[start:]
}

func (p *packet) read_str(start int, leng int) string {
	return p.line[start:(start + leng)]
}

func (p *packet) read(start int, leng int) int {
	return bin2dig(p.read_str(start, leng))
}
func (p *packet) version() int {
	return p.read(0, 3)
}

func (p *packet) id() int {
	return p.read(3, 3)
}

func (p *packet) ltid() int {
	return p.read(6, 1)
}

func (p *packet) payloadLen() int {
	return p.read(7, 15)
}

func (p *packet) payloadCount() int {
	return p.read(7, 11)
}

func (p *packet) sumVersions() int {
	sum := p.version()
	for _, pckt := range p.subPackets() {
		sum += pckt.sumVersions()
	}
	return sum
}

func (p *packet) subPackets() []packet {
	var res []packet = make([]packet, 0)
	if p.id() == 4 {
		return res
	}

	takeCount := p.ltid() == 1
	readLen := 0
	readCount := 0

	if !takeCount {
		payload := p.read_str(22, p.payloadLen())
		expectedLen := p.payloadLen()
		for {
			ln := payload[readLen:]
			pckt := packet{line: ln}
			_, leng := pckt.val()
			readLen += leng + 6
			readCount++
			res = append(res, pckt)
			if readLen >= expectedLen {
				break
			}
		}
	} else {
		payload := p.read_rest_str(18)
		expectedCount := p.payloadCount()
		for {
			ln := payload[readLen:]
			// fmt.Printf("\nStarting %d", readLen)
			pckt := packet{line: ln}
			// fmt.Printf("\nReading ln %s, ver: %d, id: %d", ln, pckt.version(), pckt.id())
			_, leng := pckt.val()
			readLen += leng + 6
			readCount++
			res = append(res, pckt)
			if takeCount && readCount == expectedCount {
				break
			}
		}
	}

	return res
}

func (p *packet) val() (int, int) {
	length := 0
	start := 6
	s := ""
	for {
		segment := p.read_str(start, 5)
		length += 5
		start += 5
		s += segment[1:]
		if string(segment[0]) == "0" {
			break
		}
	}

	return bin2dig(s), length
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

	fmt.Println(pck)

	errCheck(scanner.Err())
}

func main() {
	run("test-input.txt")
	// run("input.txt")
}
