package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	data, err := os.ReadFile("./testdata/input.txt")
	if err != nil {
		log.Fatalf("ERROR (readfile): %v", err)
	}
	fs, err := Parse(string(data))
	if err != nil {
		log.Fatalf("ERROR (parse): %v", err)
	}

	sum := SumBelowThreshold(&fs, 100_000)
	log.Printf("Sum below threshold: %d", sum)
}

type Directory struct {
	Parent *Directory
	Name   string
	Nodes  []any
}

type File struct {
	Parent *Directory
	Name   string
	Size   int64
}

func Parse(input string) (Directory, error) {
	var root Directory
	root.Name = "/"
	var currentDirectory *Directory = &root
	var currentPath []string

	tokens := make(chan string)

	go tokenize(input, tokens)

	currentToken := <-tokens

outer:
	for {
		if currentToken != "$" {
			return Directory{}, fmt.Errorf("unexpected token: %s", currentToken)
		}

		command := <-tokens
		// log.Printf("eval: '%s' in path '/%s'", command, filepath.Join(currentPath...))
		switch command {
		case "cd":
			path := <-tokens

			if path == "/" {
				currentPath = []string{}
				currentDirectory = &root
			} else if path == ".." {
				currentPath = currentPath[0 : len(currentPath)-1] // drop last element
				currentDirectory = currentDirectory.Parent
			} else {
				currentPath = append(currentPath, path)
				var newDirectory *Directory
				for _, node := range currentDirectory.Nodes {
					if n, ok := node.(*Directory); ok && n.Name == path {
						newDirectory = n
						break
					}
				}
				if newDirectory == nil {
					newDirectory := &Directory{Name: path, Parent: currentDirectory}
					currentDirectory.Nodes = append(currentDirectory.Nodes, newDirectory)
				}
				currentDirectory = newDirectory
			}
		case "ls":
			for {
				t, ok := <-tokens
				if !ok {
					break outer
				}

				switch t {
				case "$":
					// we reached the end of the output
					currentToken = t
					continue outer
				case "dir":
					n := <-tokens
					dir := Directory{Parent: currentDirectory, Name: n}
					currentDirectory.Nodes = append(currentDirectory.Nodes, &dir)
				default:
					n := <-tokens
					d, err := strconv.ParseInt(t, 10, 64)
					if err != nil {
						return Directory{}, err
					}
					file := File{Parent: currentDirectory, Name: n, Size: d}
					currentDirectory.Nodes = append(currentDirectory.Nodes, &file)
				}
			}
		}

		var ok bool
		currentToken, ok = <-tokens
		if !ok {
			break outer
		}
	}

	return root, nil
}

func tokenize(input string, tokens chan<- string) {
	defer close(tokens)

	scanner := bufio.NewScanner(strings.NewReader(input))

	for scanner.Scan() {
		line := scanner.Text()

		if commandLine, ok := strings.CutPrefix(line, "$ "); ok {
			tokens <- "$"

			if path, ok := strings.CutPrefix(commandLine, "cd "); ok {
				tokens <- "cd"
				tokens <- path
			} else if commandLine == "ls" {
				tokens <- "ls"
			} else {
				panic("unexpected command token: " + commandLine)
			}
		} else {
			s := strings.SplitN(line, " ", 2) // Either "dir NAME" or "FILESIZE NAME"
			tokens <- s[0]
			tokens <- s[1]
		}
	}
}

func Render(out io.Writer, root *Directory) error {
	return writeDirectory(out, root, 0)
}

func writeDirectory(out io.Writer, n *Directory, indent int) error {
	if _, err := fmt.Fprintf(out, "%s- %s (dir)\n", strings.Repeat(" ", indent), n.Name); err != nil {
		return err
	}

	for _, node := range n.Nodes {
		switch node := node.(type) {
		case *Directory:
			if err := writeDirectory(out, node, indent+1); err != nil {
				return err
			}
		case *File:
			fmt.Fprintf(out, "%s- %s (file, size=%d)\n", strings.Repeat(" ", indent+1), node.Name, node.Size)
		}

	}
	return nil
}

func TotalSize(root *Directory) int64 {
	var size int64
	for _, node := range root.Nodes {
		switch node := node.(type) {
		case *Directory:
			size += TotalSize(node)
		case *File:
			size += node.Size
		}
	}
	return size
}

func Walk(dir *Directory, cb func(any)) {
	for _, node := range dir.Nodes {
		cb(node)
		switch node := node.(type) {
		case *Directory:
			Walk(node, cb) // walk recursively
		}
	}
}

func SumBelowThreshold(dir *Directory, threshold int64) int64 {
	var totalSize int64
	Walk(dir, func(n any) {
		switch n := n.(type) {
		case *Directory:
			size := TotalSize(n)
			if size < threshold {
				totalSize += size
			}
		default:
		}
	})
	return totalSize
}
