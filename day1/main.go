package main

import (
	"fmt"
	"io"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	left, right := make([]int, 1000), make([]int, 1000)

	f, e := os.Open("input.csv")
	if e != nil {
		log.Panicln(e)
	}

	b, _ := io.ReadAll(f)
	s := strings.Split(string(b), "\n")

	for i := range s {
		line := strings.Split(s[i], "   ")
		left[i], _ = strconv.Atoi(line[0])
		right[i], _ = strconv.Atoi(line[1])
	}

	sort.Ints(left)
	sort.Ints(right)

	ans := 0
	for i := 0; i < 1000; i++ {
		if left[i] > right[i] {
			ans += left[i] - right[i]
		} else {
			ans += right[i] - left[i]
		}
	}

	fmt.Println(ans)
}
