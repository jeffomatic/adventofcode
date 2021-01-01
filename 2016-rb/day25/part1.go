package main

import "fmt"

func try(a int) {
	var (
		b, c, d int
		// out     []int
	)

	d = a // cpy a d
	c = 4 // cpy 4 c

L3:
	b = 633 // L3: cpy 633 b

L4:
	d++ // L4: inc d
	b-- // dec b

	// jnz b L4
	if b != 0 {
		goto L4
	}

	c-- // dec c

	// jnz c L3
	if c != 0 {
		goto L3
	}

L9:
	a = d // L9: cpy d a

L10: // L10: nop
	b = a // cpy a b
	a = 0 // cpy 0 a

L13:
	c = 2 // L13: cpy 2 c

L14:
	// L14: jnz b L16
	if b != 0 {
		goto L16
	}

	goto L21 // goto L21

L16:
	b-- // L16: dec b
	c-- // dec c

	// jnz c L14
	if c != 0 {
		goto L14
	}

	a++      // inc a
	goto L13 // goto L13

L21:
	b = 2 // L21: cpy 2 b

L22:
	// L22: jnz c L24
	if c != 0 {
		goto L24
	}

	goto L27 // goto L27

L24:
	b-- // L24: dec b

	c--      // dec c
	goto L22 // goto L22

L27: // L27: nop
	fmt.Print(b) // out b

	// jnz a L10
	if a != 0 {
		goto L10
	}

	goto L9 // goto L9
}

func main() {
	try(2)
}
