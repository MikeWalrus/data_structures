#! /usr/bin/sh
dir=$(cd -P -- "$(dirname -- "$0")" && pwd -P)
cd "$dir"/../ || exit

input=$(shuf -i 0-1000 -n 1000)
for impl in "sequential" "singly_linked" "circular"; do
    echo "$input" | chrt -f 99 ./target/release/list_stats -i ${impl}
done
