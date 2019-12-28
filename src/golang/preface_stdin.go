package main

import (
	"bufio"
	"fmt"
	"os"
)

func getchar(reader *bufio.Reader) uint8 {
	input, _ := reader.ReadString('\n')
	return uint8(input[0])
}

func main() {
	buff := make([]uint8, 20000)
	ptr := buff
	reader := bufio.NewReader(os.Stdin)
	ndx := 0
