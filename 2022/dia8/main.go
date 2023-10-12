package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func readMatrixFromFile(filePath string) ([][]int, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var matrix [][]int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		row := make([]int, len(line))
		for i, strValue := range line {
			value, err := strconv.Atoi(string(strValue))
			if err != nil {
				return nil, err
			}
			row[i] = value
		}

		matrix = append(matrix, row)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return matrix, nil
}

// check if the tree in position (x,y) is visible from the outside
// a tree is visible if there is no tree higher than it on the left,
// on the right, on the top or on the bottom
func checkIfIsVisible(matrix [][]int, x int, y int) bool {
	// if the tree is on the border of the matrix, it is visible
	if x == 0 || x == len(matrix[y])-1 || y == 0 || y == len(matrix)-1 {
		return true
	}

	visibleFromBottom, visibleFromTop := true, true
	visibleFromLeft, visibleFromRight := true, true

	// first check if the tree is visible from the left
	for i := 0; i < y; i++ {
		if matrix[x][i] >= matrix[x][y] {
			// fmt.Println("Tree in position", x, y, "is not visible from the left")
			// fmt.Println("Tree in position", x, y, "is not visible from the left because of tree in position", x, i, "with height", matrix[x][i])
			visibleFromLeft = false
		}
	}

	// then check if the tree is visible from the right
	for i := y + 1; i < len(matrix[x]); i++ {
		if matrix[x][i] >= matrix[x][y] {
			visibleFromRight = false
		}
	}

	// then check if the tree is visible from the top
	for i := 0; i < x; i++ {
		if matrix[i][y] >= matrix[x][y] {
			visibleFromTop = false
		}
	}

	// then check if the tree is visible from the bottom
	for i := x + 1; i < len(matrix); i++ {
		if matrix[i][y] >= matrix[x][y] {
			visibleFromBottom = false
		}
	}

	return visibleFromLeft || visibleFromRight || visibleFromTop || visibleFromBottom
}

func calculateScenicScore(grid [][]int, row, col, height int) int {
	viewUp := 1
	viewDown := 1
	viewLeft := 1
	viewRight := 1

	// Look up
	for i := row - 1; (i >= 0 && grid[i][col] < height) && !isBorder(grid, i, col); i-- {
		viewUp++
	}

	// Look down
	for i := row + 1; i < len(grid) && grid[i][col] < height && !isBorder(grid, i, col); i++ {
		viewDown++
	}

	// Look left
	for j := col - 1; j >= 0 && grid[row][j] < height && !isBorder(grid, row, j); j-- {
		viewLeft++
	}

	// Look right
	for j := col + 1; j < len(grid[0]) && grid[row][j] < height && !isBorder(grid, row, j); j++ {
		viewRight++
	}

	fmt.Println("X:", row, "Y:", col, "Height:", height)
	fmt.Println("Scenic score:", viewUp*viewDown*viewLeft*viewRight)
	fmt.Println("View up:", viewUp)
	fmt.Println("View down:", viewDown)
	fmt.Println("View left:", viewLeft)
	fmt.Println("View right:", viewRight)

	// Calculate scenic score by multiplying view distances
	return viewUp * viewDown * viewLeft * viewRight
}

func isBorder(grid [][]int, row, col int) bool {
	return row == 0 || row == len(grid)-1 || col == 0 || col == len(grid[0])-1
}

func main() {

	trees, _ := readMatrixFromFile("matrix.txt")
	// count how many trees are visible from the outside
	highestScore := 0
	for y := 0; y < len(trees); y++ {
		for x := 0; x < len(trees[y]); x++ {
			if calculateScenicScore(trees, x, y, trees[x][y]) > highestScore {
				highestScore = calculateScenicScore(trees, x, y, trees[x][y])
			}
		}
	}

	fmt.Println("The highest scenic score is", highestScore)

}
