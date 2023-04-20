package main

import (
	"bufio"
	"fmt"
	"os"
)

func itemValue(item rune) int {
	if item >= 'a' && item <= 'z' {
		return int(item-'a') + 1
	} else {
		return int(item-'A') + 27
	}
}

func part1() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	scanner := bufio.NewScanner(file)
	score := 0
	for scanner.Scan() {
		line := scanner.Text()
		items := make(map[rune]bool)
		for i, c := range line {
			if i < len(line)/2 {
				items[c] = true
			} else {
				if items[c] == true {
					score += itemValue(c)
					break
				}
			}
		}
	}

	fmt.Printf("\nScore: %d", score)
}

func part2() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	scanner := bufio.NewScanner(file)
	lineNum := 0
	score := 0
	items := make(map[rune]int)
	for scanner.Scan() {
		line := scanner.Text()

		for _, c := range line {
			items[c] = items[c] | 1<<lineNum
		}
		lineNum += 1
		if lineNum == 3 {
			for k, v := range items {
				if v == 7 {
					score += itemValue(k)
					break
				}
			}
			lineNum = 0
			items = make(map[rune]int)
		}
	}

	fmt.Printf("\nScore: %d", score)
}

func main() {
	part1()
	part2()
}
