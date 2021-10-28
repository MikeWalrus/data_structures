#! /usr/bin/sh
dir=$(cd -P -- "$(dirname -- "$0")" && pwd -P)
cd "$dir"/../ || exit

od -An -t fD -N800 </dev/urandom | ./target/release/list_partition
