// Get these dependencies first by running the following commands:

// go mod init csvhead
// go get github.com/sqweek/dialog


package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"strings"

	"github.com/sqweek/dialog"
)

func main() {
	// Open native file browser, filtered to CSV files
	filePath, err := dialog.File().
		Filter("CSV files", "csv").
		Title("Select a CSV file").
		Load()
	if err != nil {
		fmt.Println("No file selected, exiting.")
		return
	}

	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	reader := csv.NewReader(file)
	records, err := reader.ReadAll()
	if err != nil {
		fmt.Println("Error reading CSV:", err)
		return
	}

	if len(records) == 0 {
		fmt.Println("CSV file is empty.")
		return
	}

	printHead(records, 5)
}

// printHead mimics pandas' df.head() — prints header + first n rows, column-aligned
func printHead(records [][]string, n int) {
	header := records[0]
	rows := records[1:]

	if n > len(rows) {
		n = len(rows)
	}
	rows = rows[:n]

	// Compute max width per column for alignment
	widths := make([]int, len(header))
	for i, h := range header {
		widths[i] = len(h)
	}
	for _, row := range rows {
		for i, val := range row {
			if i < len(widths) && len(val) > widths[i] {
				widths[i] = len(val)
			}
		}
	}

	printRow(header, widths)
	printSeparator(widths)
	for _, row := range rows {
		printRow(row, widths)
	}
}

func printRow(row []string, widths []int) {
	var sb strings.Builder
	for i, val := range row {
		sb.WriteString(fmt.Sprintf("%-*s  ", widths[i], val))
	}
	fmt.Println(sb.String())
}

func printSeparator(widths []int) {
	var sb strings.Builder
	for _, w := range widths {
		sb.WriteString(strings.Repeat("-", w) + "  ")
	}
	fmt.Println(sb.String())
}