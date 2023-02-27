package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	shapeScores := map[string]int{
		"X": 1,
		"Y": 2,
		"Z": 3,
	}

	myShapeValues := map[string]int{
		"X": 1,
		"Z": 2,
		"Y": 3,
	}

	opponentShapeValues := map[string]int{
		"A": 1,
		"C": 2,
		"B": 3,
	}

	scanner := bufio.NewScanner(file)
	score := 0
	for scanner.Scan() {
		line := scanner.Text()
		moves := strings.Fields(line)

		diff := opponentShapeValues[moves[0]] - myShapeValues[moves[1]]
		if diff == 1 || diff == -2 { // win condition
			score += 6
		} else if diff == 0 {
			score += 3
		}

		score += shapeScores[moves[1]]
	}

	fmt.Printf("\nScore: %d", score)
}
