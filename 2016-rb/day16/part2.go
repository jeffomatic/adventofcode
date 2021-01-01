package main

import "fmt"

func toBool(s string) []bool {
	res := make([]bool, 0, len(s))
	for _, c := range s {
		if c == '1' {
			res = append(res, true)
		} else {
			res = append(res, false)
		}
	}
	return res
}

func toString(s []bool) string {
	res := make([]byte, 0, len(s))
	for _, b := range s {
		if b {
			res = append(res, '1')
		} else {
			res = append(res, '0')
		}
	}
	return string(res)
}

func reverse(s []bool) {
	for i := 0; i < len(s)/2; i++ {
		other := len(s) - 1 - i
		s[i], s[other] = s[other], s[i]
	}
}

func flip(s []bool) {
	for i, b := range s {
		s[i] = !b
	}
}

func dragon(a []bool) []bool {
	b := make([]bool, 0, len(a))
	b = append(b, a...)
	reverse(b)
	flip(b)

	res := make([]bool, 0, 2*len(a)+1)
	res = append(res, a...)
	res = append(res, false)
	res = append(res, b...)

	return res
}

func checksum(s []bool) []bool {
	res := make([]bool, 0, len(s)/2)
	for i := 0; i < len(s); i += 2 {
		res = append(res, s[i] == s[i+1])
	}

	if len(res)%2 == 1 {
		return res
	}

	return checksum(res)
}

func diskData(initial []bool, want int) []bool {
	res := initial
	for len(res) < want {
		res = dragon(res)
	}
	return res[0:want]
}

func main() {
	fmt.Println(toString(checksum(diskData(toBool("01111001100111011"), 35651584))))
}
