package main

import "fmt"

const numElves = 3004953

type elf struct {
	id          int
	left, right *elf
}

func main() {
	elves := make([]elf, numElves, numElves)

	for i := 1; i < numElves-1; i++ {
		elves[i].id = i + 1
		elves[i].right = &elves[i-1]
		elves[i].left = &elves[i+1]
	}

	elves[0].id = 1
	elves[0].left = &elves[1]
	elves[0].right = &elves[numElves-1]

	elves[numElves-1].id = numElves
	elves[numElves-1].left = &elves[0]
	elves[numElves-1].right = &elves[numElves-2]

	cur := &elves[0]
	opp := &elves[numElves/2]
	odd := numElves%2 == 1

	for {
		opp.right.left, opp.left.right = opp.left, opp.right
		opp = opp.left
		if cur == opp {
			break
		}

		cur = cur.left
		if odd {
			opp = opp.left
		}
		odd = !odd
	}

	fmt.Println(cur.id)
}
