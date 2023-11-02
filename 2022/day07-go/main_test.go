package main

import (
	"strings"
	"testing"

	approvals "github.com/approvals/go-approval-tests"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const INPUT = `$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k`

const RENDERED = `- / (dir)
- a (dir)
  - e (dir)
   - i (file, size=584)
  - f (file, size=29116)
  - g (file, size=2557)
  - h.lst (file, size=62596)
- b.txt (file, size=14848514)
- c.dat (file, size=8504156)
- d (dir)
  - j (file, size=4060174)
  - d.log (file, size=8033020)
  - d.ext (file, size=5626152)
  - k (file, size=7214296)`

func TestParse(t *testing.T) {
	fs, err := Parse(INPUT)
	require.NoError(t, err)

	var buf strings.Builder
	require.NoError(t, Render(&buf, &fs))
	approvals.VerifyString(t, buf.String())

	assert.EqualValues(t, 48_381_165, TotalSize(&fs))
}

func TestBelow100k(t *testing.T) {
	fs, err := Parse(INPUT)
	require.NoError(t, err)

	totalSize := SumBelowThreshold(&fs, 100_000)
	assert.EqualValues(t, 95437, totalSize)
}

func TestFindSmallestDeletable(t *testing.T) {
	fs, err := Parse(INPUT)
	require.NoError(t, err)

	sizeSmallestDeletable := FindSmallestDeletable(&fs, DISK_SIZE, UPDAE_SIZE)
	assert.EqualValues(t, 24_933_642, sizeSmallestDeletable)
}
