package internal

import (
	"os"
	"strings"
)

func MustReadFile(path string) []string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	return strings.Split(string(data), "\n")
}
