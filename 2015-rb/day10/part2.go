package main

import "fmt"

func encode(s []rune) []rune {
	var (
		current rune
		count   int
		res     []rune
	)

	for _, r := range s {
		if current == 0 {
			current = r
			count = 1
			continue
		}

		if r != current {
			res = append(res, rune(count+48), current)
			current = r
			count = 1
			continue
		}

		count++
	}

	return append(res, rune(count+48), current)
}

func main() {
	input := []rune("1113122113")
	for i := 0; i < 50; i++ {
		input = encode(input)
	}
	fmt.Println(len(input))
}
