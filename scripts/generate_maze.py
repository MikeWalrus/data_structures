#! /usr/bin/python3

from random import randrange, choice

def get_random_empty_pos(width, height, board):
    while True:
        ret = (randrange(height), randrange(width))
        if board[ret[0]][ret[1]] == 0:
            return ret

def main():
    width = randrange(10, 50)
    height = width + randrange(-4, 4)
    board = [[choice([0, 0, 0, 1]) for i in range(width)] for i in range(height)]
    entry_pos = get_random_empty_pos(width, height, board)
    exit_pos = get_random_empty_pos(width, height, board)

    print(height, width)
    print(entry_pos[0] + 1, entry_pos[1] + 1, exit_pos[0] + 1, exit_pos[1] + 1)
    for line in board:
        print(" ".join(map(str, line)))

if __name__ == "__main__":
    main()
