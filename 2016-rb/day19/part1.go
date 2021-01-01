package main

import "fmt"

const numElves = 3004953

type elf struct {
	id   int
	left *elf
}

func main() {
	elves := make([]elf, numElves, numElves)

	for i := 0; i < numElves-1; i++ {
		elves[i].id = i + 1
		elves[i].left = &elves[i+1]
	}

	elves[numElves-1].id = numElves
	elves[numElves-1].left = &elves[0]

	var cur *elf
	for cur = &elves[0]; cur != cur.left; cur = cur.left {
		cur.left = cur.left.left
	}

	fmt.Println(cur.id)
}
