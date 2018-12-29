#!/bin/sh

cargo build --release

rm -f out.stacks

sudo dtrace -c './target/release/day09' -o out.stacks -n 'profile-997 /execname == "day09"/ { @[ustack(100)] = count(); }'

# render with brendangreggs FlameGraph scripts
~/p/FlameGraph/stackcollapse.pl out.stacks | ~/p/FlameGraph/flamegraph.pl > pretty.svg

echo "pretty.svg is ready"