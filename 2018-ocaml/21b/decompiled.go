package main

import "fmt"

func main() {
	a := 0
	b := 0
	c := 0
	d := 0
	e := 0

B:
	// 06: b = c | 0x10000
	// 07: c = 1250634
	b = c | 0x10000 // 65536
	c = 1250634

C:
	// 08: e = b & 0xFF
	// 09: c = c + e
	// 10: c = c & 0xFFFFFF
	// 11: c = c * 65899
	// 12: c = c & 0xFFFFFF
	// 13: e = 256 > b
	// 14: goto e + 15
	// 15: goto 17
	// 16: goto 28
	e = b & 0xFF // get first 8 bits of b
	c = c + e
	c = c & 0xFFFFFF // get first 24 bits of c
	c = c * 65899
	c = c & 0xFFFFFF // get first 24 bits of c

	// We'll halt if b < 256 and c == a
	if b < 256 {
		goto F
	}

	// 17: e = 0
	e = 0

D:
	// 18: d = e + 1
	// 19: d = d * 256
	// 20: d = d > b
	// 21: goto d + 22
	// 22: goto 24
	// 23: goto 26
	d = e + 1
	d = d * 256
	if b < d {
		goto E
	}

	// 24: e = e + 1
	// 25: goto D
	e = e + 1
	goto D

E:
	// 26: b = e
	// 27: goto C
	b = e
	goto C

F:
	// 28: e = c == a
	// 29: goto e + 30
	// 30: goto B
	fmt.Println(c)
	if c != a {
		goto B
	}

	// We get here if b < 256 and c == a
	fmt.Println("HALTED")
}
