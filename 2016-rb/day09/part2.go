package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func reverse(b []byte) {
	for i := 0; i < len(b)/2; i++ {
		other := len(b) - i - 1
		b[i], b[other] = b[other], b[i]
	}
}

// backread consumes n bytes from the end of a slice of bytes. The result is
// returned in the order consumed (from back to front). The unconsumed portion
// of buf is also returned.
func backread(buf []byte, n int) (read []byte, newBuf []byte) {
	read = make([]byte, n, n)
	for i := 0; i < n; i++ {
		read[i] = buf[len(buf)-i-1]
	}
	newBuf = buf[0 : len(buf)-n]
	return
}

// backread_until consumes from the back of buf until it reaches a value equal
// to end. The result is returned in the order consumed (from back to front).
// The unconsumed portion, not counting end, is also returned.
func backread_until(buf []byte, end byte) (read []byte, newBuf []byte) {
	for i := len(buf) - 1; i >= 0; i-- {
		if buf[i] == end {
			return
		}
		read = append(read, buf[i])
		newBuf = buf[0 : i-1]
	}

	panic("Should never reach here")
}

func main() {
	buf, err := ioutil.ReadFile("./input")
	if err != nil {
		log.Fatal(err)
	}

	if buf[len(buf)-1] == '\n' {
		buf = buf[0 : len(buf)-1]
	}
	reverse(buf)

	var (
		read []byte
		res  int
	)

	for len(buf) > 0 {
		read, buf = backread(buf, 1)

		if read[0] != '(' {
			res++
			continue
		}

		read, buf = backread_until(buf, ')')
		toks := strings.Split(string(read), "x")
		size, _ := strconv.Atoi(toks[0])
		repeat, _ := strconv.Atoi(toks[1])

		read, buf = backread(buf, size)
		reverse(read)
		for i := 0; i < repeat; i++ {
			buf = append(buf, read...)
		}
	}

	fmt.Println(res)
}
