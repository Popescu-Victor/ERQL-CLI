package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Print(">> ")

		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Println("Error reading input:", err)
			break
		}

		// Trim the trailing newline and any surrounding whitespace
		input = strings.TrimSpace(input)

		if input == "exit" {
			break
		}

		args := strings.Fields(input) // splits on whitespace, ignores extra spaces
		fmt.Println(args)
	}
}