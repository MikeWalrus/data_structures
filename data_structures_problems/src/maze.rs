use std::{
    collections::{HashSet, VecDeque},
    convert::identity,
    fmt::Display,
    io::{self, BufRead},
    str::SplitWhitespace,
    usize,
};

use data_structures::{
    queue::{Queue, SeqQueue},
    stack::{SeqStack, Stack},
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

    let mut board = read_board(height, width, lines);
    board[entry.0][entry.1] = Block::Entry;
    board[exit.0][exit.1] = Block::Exit;
    let maze = Maze { board, entry, exit };
    println!("{}", maze);
    match (maze.solve_bfs(), maze.solve_dfs()) {
        (None, None) => {
            eprintln!("Unsolvable");
            std::process::exit(1)
        }
        (None, Some(_)) => panic!("bfs failed"),
        (Some(_), None) => panic!("dfs failed"),
        (Some(bfs_solution), Some(dfs_solution)) => {
            maze.print_solution(&bfs_solution);
            maze.print_solution(&dfs_solution);
            assert!(maze.is_solved(dfs_solution));
            assert!(maze.is_solved(bfs_solution));
        }
    }
}

fn read_board(height: usize, width: usize, mut lines: io::Lines<io::StdinLock>) -> Board {
    let mut board: Vec<Vec<Block>> = Vec::with_capacity(height + 2);
    add_horizontal_boundary(&mut board, width);
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
    add_horizontal_boundary(&mut board, width);
    board
}

fn add_horizontal_boundary(maze: &mut Vec<Vec<Block>>, width: usize) {
    maze.push(vec![Block::Obstacle; width + 2]);
}

#[derive(Debug, Clone)]
enum Block {
    Entry,
    Exit,
    Empty,
    Obstacle,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Block::Empty => '.',
                Block::Obstacle => '#',
                Block::Entry => '<',
                Block::Exit => '>',
            }
        )
    }
}

type Board = Vec<Vec<Block>>;

type Coord = (usize, usize);

#[derive(Debug)]
struct Maze {
    board: Board,
    entry: Coord,
    exit: Coord,
}

struct Step {
    coord: Coord,
    count: u32,
}

impl Iterator for Step {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j) = self.coord;

        let next_coord = match self.count {
            0 => (i - 1, j),
            1 => (i, j + 1),
            2 => (i + 1, j),
            3 => (i, j - 1),
            _ => return None,
        };
        self.count += 1;
        Some(next_coord)
    }
}

impl Step {
    fn with_coord(coord: Coord) -> Self {
        Step { coord, count: 0 }
    }
}

impl Maze {
    fn solve_dfs(&self) -> Option<Vec<Coord>> {
        let stack = self.dfs()?;
        Some(
            stack
                .list()
                .iter()
                .map(|Step { coord, count: _ }| *coord)
                .collect(),
        )
    }

    fn dfs(&self) -> Option<SeqStack<Step>> {
        let mut stack: SeqStack<Step> = SeqStack::new();
        let mut visited = HashSet::<Coord>::new();
        stack.push(Step::with_coord(self.entry));
        loop {
            let current = stack.peek_mut()?;
            visited.insert(current.coord);
            if current.coord == self.exit {
                break;
            }
            if let Some(next_step) = loop {
                match current.next() {
                    Some(next) => match self.board[next.0][next.1] {
                        Block::Obstacle => continue,
                        _ => {
                            if !visited.contains(&next) {
                                break Some(Step::with_coord(next));
                            }
                        }
                    },
                    None => {
                        stack.pop();
                        break None;
                    }
                }
            } {
                stack.push(next_step)
            }
        }
        Some(stack)
    }

    fn is_solved(&self, solution: Vec<Coord>) -> bool {
        assert!(solution[0] == self.entry);
        assert!(solution.last().unwrap() == &self.exit);
        solution
            .windows(2)
            .map(|i| {
                let a = i[0];
                let b = i[1];
                !matches!(self.board[a.0][a.1], Block::Obstacle) && is_adjacent(a, b)
            })
            .all(identity)
    }

    fn print_solution(&self, solution: &[Coord]) {
        if solution.len() <= 2 {
            println!("Trivial.");
            return;
        }
        let mut chars: Vec<Vec<char>> = Vec::with_capacity(self.board.len());
        chars.extend(self.board.iter().map(|line| {
            let mut v = Vec::with_capacity(self.board[0].len());
            v.extend(line.iter().map(|block| match block {
                Block::Entry => '<',
                Block::Exit => '>',
                Block::Empty => '.',
                Block::Obstacle => '#',
            }));
            v
        }));
        solution.windows(3).fold(
            Orientation::from_coords(solution[0], solution[1]),
            |prev, window| {
                let current = Orientation::from_coords(window[1], window[2]);
                chars[window[1].0][window[1].1] = match (&prev, &current) {
                    (Orientation::North, Orientation::North)
                    | (Orientation::South, Orientation::South) => '│',
                    (Orientation::North, Orientation::West)
                    | (Orientation::East, Orientation::South) => '└',
                    (Orientation::North, Orientation::East)
                    | (Orientation::West, Orientation::South) => '┘',
                    (Orientation::South, Orientation::West)
                    | (Orientation::East, Orientation::North) => '┌',
                    (Orientation::South, Orientation::East)
                    | (Orientation::West, Orientation::North) => '┐',
                    (Orientation::West, Orientation::West)
                    | (Orientation::East, Orientation::East) => '─',
                    _ => 'X',
                };
                current
            },
        );
        for line in chars {
            for c in line {
                print!("{}", c)
            }
            println!()
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.board {
            for block in line {
                write!(f, "{}", block)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

fn get_next_pair(i: &mut SplitWhitespace) -> (usize, usize) {
    let mut it = i.map(str::parse::<usize>);
    (it.next().unwrap().unwrap(), it.next().unwrap().unwrap())
}

fn is_adjacent(a: Coord, b: Coord) -> bool {
    let diff = (a.0.wrapping_sub(b.0), a.1.wrapping_sub(b.1));
    const MINUS_ONE: usize = usize::MAX;
    matches!(diff, (MINUS_ONE, 0) | (1, 0) | (0, MINUS_ONE) | (0, 1))
}

enum Orientation {
    North,
    South,
    West,
    East,
}

impl Orientation {
    fn from_coords(a: Coord, b: Coord) -> Self {
        match a.0.cmp(&b.0) {
            std::cmp::Ordering::Less => Orientation::North,
            std::cmp::Ordering::Equal => match a.1.cmp(&b.1) {
                std::cmp::Ordering::Less => Orientation::West,
                _ => Orientation::East,
            },
            std::cmp::Ordering::Greater => Orientation::South,
        }
    }
}

impl Maze {
    fn solve_bfs(&self) -> Option<Vec<Coord>> {
        let mut prev_steps: Vec<Vec<Option<Coord>>> =
            vec![vec![None; self.board[0].len()]; self.board.len()];
        let mut queue: SeqQueue<Step> = SeqQueue::new();
        queue.push(Step::with_coord(self.entry));
        prev_steps[self.entry.0][self.entry.1] = Some(self.entry);
        loop {
            let current = queue.pop_front()?;
            let current_coord = current.coord;

            if current_coord == self.exit {
                break;
            }

            for next_coord in current {
                if prev_steps[next_coord.0][next_coord.1].is_none()
                    && !matches!(self.board[next_coord.0][next_coord.1], Block::Obstacle)
                {
                    prev_steps[next_coord.0][next_coord.1] = Some(current_coord);
                    queue.push(Step::with_coord(next_coord));
                }
            }
        }
        let mut solution = VecDeque::<Coord>::new();
        let mut i = self.exit;
        while i != self.entry {
            solution.push_front(i);
            i = prev_steps[i.0][i.1].unwrap()
        }
        solution.push_front(self.entry);
        Some(Vec::from(solution))
    }
}
