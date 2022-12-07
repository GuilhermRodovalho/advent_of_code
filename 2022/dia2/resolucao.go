package main

import (
	"fmt"
	"os"
	"strings"
)

var shapeCanBeat = map[string]string{
	"A": "Z",
	"B": "X",
	"C": "Y",
	"X": "C",
	"Y": "A",
	"Z": "B",
}

var pointsForShape = map[string]int{
	"X": 1,
	"Y": 2,
	"Z": 3,
	"A": 1,
	"B": 2,
	"C": 3,
}

// Helpers for part2
var draws = map[string]string{
	"A": "A",
	"B": "B",
	"C": "C",
}

var newShapeCanBeat = map[string]string{
	"A": "C",
	"B": "A",
	"C": "B",
}

var shapeIsBeated = map[string]string{
	"C": "A",
	"A": "B",
	"B": "C",
}

func main() {
	// Parse data
	data, _ := os.ReadFile("./finalInput.txt")
	lines := strings.Split(string(data), "\n")

	points := 0

	// part1
	// for _, line := range lines {
	// 	actions := strings.Split(line, " ")
	// 	elfShape, myShape := actions[0], actions[1]

	// 	if shapeCanBeat[myShape] == elfShape {
	// 		points += 6
	// 	} else if shapeCanBeat[elfShape] != myShape {
	// 		points += 3
	// 	}

	// 	points += pointsForShape[myShape]
	// }

	// fmt.Println(points)

	points = 0
	for _, line := range lines {
		actions := strings.Split(line, " ")
		elfShape, objective := actions[0], actions[1]

		if objective == "X" {
			myShape := newShapeCanBeat[elfShape]

			points += pointsForShape[myShape]
		} else if objective == "Y" {
			points += 3 + pointsForShape[elfShape]
		} else {
			myShape := shapeIsBeated[elfShape]

			points += 6 + pointsForShape[myShape]
		}

	}

	fmt.Println(points)

}
