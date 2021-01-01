package main

import "fmt"

func try(a int) {
	b := 0
	c := 0
	prevc := -1
	all := make(map[int]struct{})

outer:
	for {
		b = c | 0x10000 // 65536
		c = 1250634

		c += b & 0xFF // add first 8 bits of b to c
		c &= 0xFFFFFF // get first 24 bits of c
		c *= 65899
		c &= 0xFFFFFF // get first 24 bits of c

		for {
			if (b < 256) {
				if _, ok := all[c]; ok {
					fmt.Println(prevc)
					return
				}

				prevc = c
				all[c] = struct{}{}
				if c == a {
					return
				}

				continue outer
			}

			b /= 256

			c += b & 0xFF // add first 8 bits of b to c
			c &= 0xFFFFFF // get first 24 bits of c
			c *= 65899
			c &= 0xFFFFFF // get first 24 bits of c
		}
	}
}


func main() {
		try(0)
}
