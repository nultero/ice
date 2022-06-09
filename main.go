package main

import (
	"errors"
	"fmt"
	"io"
	"math/rand"
	"os"
)

var iceChars = []byte{
	24, 25, 26, 27, 31, 32, 33, 38, 39,
	43, 44, 45, 50, 51, 60, 61, 62, 63,
	67, 68, 69, 74, 75, 79, 80, 81, 86,
	87, 96, 97, 98, 99, 103, 104, 105,
	110, 111, 115, 116, 117, 122, 123,
}

func getSeed() int64 {
	fptr, err := os.Open("/dev/urandom")
	if err != nil {
		panic(err)
	}
	bytes := make([]byte, 5)
	_, err = fptr.Read(bytes)
	if err != nil {
		panic(err)
	}

	sum := byte(5)
	for _, b := range bytes {
		sum += b
	}

	return int64(sum)
}

func main() {

	s := getSeed()
	rand.Seed(s)
	idx := rand.Intn(len(iceChars))

	stdin := os.Stdin
	buf := make([]byte, 150)

	for {
		n, err := stdin.Read(buf)
		if n == 0 || errors.Is(err, io.EOF) {
			break
		}

		if err != nil {
			panic(err)
		}

		for _, b := range buf {
			fmt.Print(rfmt(b, idx))
			idx++
			if idx == len(iceChars) {
				idx = 0
			}
		}
	}
}

const (
	st = "\x1b["
	en = "\x1b[0m"
)

func rfmt(b byte, idx int) string {
	return fmt.Sprintf(
		"%v38;5;%vm%c%v",
		st,
		iceChars[idx],
		b,
		en,
	)
}
