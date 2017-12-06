package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	input := readInput()
	steps := part1(input)
	fmt.Println(steps)
}

func part1(input []int) (steps int) {
	i := 0
	for {
		if i < 0 || i > (len(input)-1) {
			return
		}
		steps++
		j := i
		tmp := input[i]
		i += input[i]
		if tmp > 2 {
			input[j]--
		} else {
			input[j]++
		}
	}
}

func readInput() []int {
	var input []int
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		i, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		input = append(input, i)
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "reading standard input:", err)
	}
	return input
}
