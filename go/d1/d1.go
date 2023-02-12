package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	scanner := bufio.NewScanner(file)

	max := 0
	sum := 0
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			if sum > max {
				max = sum
			}
			sum = 0
			continue
		}
		value, err := strconv.Atoi(line)
		if err != nil {
			fmt.Println(err)
			return
		}
		sum += value
	}

	if sum > max {
		max = sum
	}

	fmt.Printf("\nMax: %d", max)
}
