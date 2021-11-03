#! /usr/bin/bash

dir=$(cd -P -- "$(dirname -- "$0")" && pwd -P)
cd "$dir" || exit

for maze_input in ./maze/*; do
    ../target/release/maze <"${maze_input}" || exit 1 # These should be solvable.
done

for _ in {1..100}; do
    ./generate_maze.py | ../target/release/maze ||
     { [ $? -ne 1 ] && exit 1; } # Unsolvable mazes pass the test.
done
