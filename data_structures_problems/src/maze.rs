use std::{
    io::{self, BufRead},
    str::SplitWhitespace,
};

fn main() {
    let stdio = io::stdin();
    let lock = stdio.lock();
    let mut lines = lock.lines();
    let line = lines.next().unwrap().unwrap();
    let (height, width) = get_next_pair(&mut line.split_whitespace());

    let line = lines.next().unwrap().unwrap();
    let mut i = line.split_whitespace();
    let entry = get_next_pair(&mut i);
    let exit = get_next_pair(&mut i);

    let board = read_board(height, width, lines);
    let maze = Maze { board, entry, exit };
}

fn read_board(height: usize, width: usize, mut lines: io::Lines<io::StdinLock>) -> Board {
    let mut board: Vec<Vec<Block>> = Vec::with_capacity(height + 2);
    add_vertical_boundary(&mut board, width);
    for _ in 0..height {
        let line = lines.next().unwrap().unwrap();
        let mut vec: Vec<Block> = Vec::with_capacity(width + 2);
        vec.push(Block::Obstacle);
        vec.extend(
            line.split_whitespace()
                .map(|s| match s.parse::<u32>().unwrap() {
                    0 => Block::Empty,
                    1 => Block::Obstacle,
                    _ => panic!(),
                })
                .take(width),
        );
        vec.push(Block::Obstacle);
        assert_eq!(vec.len(), width + 2);
        board.push(vec)
    }
    add_vertical_boundary(&mut board, width);
    println!("{:?}", board);
    board
}

fn add_vertical_boundary(maze: &mut Vec<Vec<Block>>, width: usize) {
    maze.push(vec![Block::Obstacle; width + 2]);
}

#[derive(Debug, Clone)]
enum Block {
    Empty,
    Obstacle,
}

type Board = Vec<Vec<Block>>;

#[derive(Debug)]
struct Maze {
    board: Board,
    entry: (usize, usize),
    exit: (usize, usize),
}

fn get_next_pair(i: &mut SplitWhitespace) -> (usize, usize) {
    let mut it = i.map(str::parse::<usize>);
    (it.next().unwrap().unwrap(), it.next().unwrap().unwrap())
}
