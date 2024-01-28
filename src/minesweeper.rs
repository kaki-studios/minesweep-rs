use core::panic;
use std::process::exit;

use crate::BOARD_HEIGHT;
use crate::BOARD_WIDTH;
use crate::MINE_COUNT;
use rand::*;

pub const ADJACENTS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

#[derive(Debug)]
pub struct Minesweeper {
    pub board: [[Tile; BOARD_WIDTH]; BOARD_HEIGHT],
    mines: [(usize, usize); MINE_COUNT],
}

impl Minesweeper {
    pub fn _new() -> Self {
        let board = [[Tile::Unknown(false); BOARD_WIDTH]; BOARD_HEIGHT];
        let mut mines = [(0, 0); MINE_COUNT];
        let mut rng = rand::thread_rng();
        let mut minestwo = mines.clone();
        for (i, mine) in mines.iter_mut().enumerate() {
            let mut random = (
                rng.gen_range(0..BOARD_WIDTH),
                rng.gen_range(0..BOARD_HEIGHT),
            );
            while minestwo.contains(&random) {
                random = (
                    rng.gen_range(0..BOARD_WIDTH),
                    rng.gen_range(0..BOARD_HEIGHT),
                );
            }
            minestwo[i] = random;
            *mine = random;
        }
        Self { board, mines }
    }
    pub fn new_with_guess(guess: (usize, usize)) -> Self {
        assert!(guess.0 < BOARD_HEIGHT);
        assert!(guess.1 < BOARD_WIDTH);

        let board = [[Tile::Unknown(false); BOARD_WIDTH]; BOARD_HEIGHT];
        let mut mines = [(0, 0); MINE_COUNT];
        let mut rng = rand::thread_rng();
        let mut prev_mines: Vec<(usize, usize)> = Vec::new();
        for mine in mines.iter_mut() {
            let mut candidate: (usize, usize);
            'iter: loop {
                candidate = (
                    rng.gen_range(0..BOARD_WIDTH),
                    rng.gen_range(0..BOARD_HEIGHT),
                );
                let mut extended = prev_mines.clone();
                extended.push(candidate);
                if candidate != guess
                    && Self::adjacent_mines(extended, guess) == 0
                    && !prev_mines.contains(&candidate)
                {
                    prev_mines.push(candidate);

                    *mine = candidate;
                    break 'iter;
                }
            }
        }

        let mut ms = Self { board, mines };
        ms.play_turn(MoveType::Dig(guess.0, guess.1));
        ms
    }

    pub fn print_board(&self) {
        print!("{}", crate::CLEAR_CODE);
        let line = "-".repeat(BOARD_WIDTH * 4 + 4);
        print!("   ");
        //first row
        for j in 0..BOARD_WIDTH {
            if j < 10 {
                print!("|  {j}");
            } else {
                print!("| {j}");
            }
        }
        println!("| ");
        println!("{line}");

        //other rows
        for (i, row) in self.board.iter().enumerate() {
            if i < 10 {
                print!("{i}  ");
            } else {
                print!("{i} ");
            }
            for tile in row {
                let sym = match tile {
                    Tile::Unknown(false) => " ".to_string(),
                    Tile::Unknown(true) => "X".to_string(),
                    Tile::Known(n) => {
                        if n < &9 {
                            n.to_string()
                        } else {
                            panic!("too many adjacents: {n}");
                        }
                    }
                };
                print!("| {sym} ");
            }
            println!("| ");
            println!("{line}");
        }
    }

    pub fn play_turn(&mut self, action: MoveType) {
        use MoveType as M;
        use Tile as T;
        match action {
            M::Flag(y, x) => {
                if let T::Unknown(val) = self.board[y][x] {
                    self.board[y][x] = T::Unknown(!val);
                }
            }
            M::Dig(y, x) => {
                if !self.mines.contains(&(y, x)) {
                    if let T::Unknown(false) = self.board[y][x] {
                        self.flood_fill((y, x));
                    }
                } else {
                    eprintln!("tried to dig at {:?}, but there was a mine!", (y, x));
                    panic!("you lost!! idk how to restart the game yet!");
                }

                let unknowns = self
                    .board
                    .iter()
                    .flatten()
                    .filter(|x| if let T::Unknown(_) = x { true } else { false })
                    .map(|x| *x)
                    .collect::<Vec<Tile>>();
                //if all mines are flagged and there are no unflagged
                //this isn't an elegant way to do it
                if unknowns.len() == MINE_COUNT && !unknowns.contains(&T::Unknown(false)) {
                    println!("you won the game!!");
                    println!("idk how to restart so i\'ll just exit");
                    exit(0);
                }
            }
        }
    }

    fn adjacent_mines(mines: Vec<(usize, usize)>, coords: (usize, usize)) -> u8 {
        let mut count = 0;
        for adj in ADJACENTS {
            let pos = (coords.0 as i32 + adj.0, coords.1 as i32 + adj.1);
            if mines.contains(&(pos.0 as usize, pos.1 as usize)) {
                count += 1;
            }
        }
        count
    }
    fn flood_fill(&mut self, coords: (usize, usize)) {
        if !(0..BOARD_HEIGHT).contains(&coords.0) || !(0..BOARD_WIDTH).contains(&coords.1) {
            return;
        }
        if let Tile::Unknown(false) = self.board[coords.0][coords.1] {
            let adj = Self::adjacent_mines(self.mines.to_vec(), (coords.0, coords.1));
            self.board[coords.0][coords.1] = Tile::Known(adj);
            if adj == 0 {
                self.flood_fill((coords.0 + 1, coords.1));
                if coords.0 > 0 {
                    self.flood_fill((coords.0 - 1, coords.1));
                }
                self.flood_fill((coords.0, coords.1 + 1));
                if coords.1 > 0 {
                    self.flood_fill((coords.0, coords.1 - 1));
                }
            }
        }
    }
}

///Unknown and whether it's flagged
///and Known and how many adjacents
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Unknown(bool),
    Known(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum MoveType {
    Flag(usize, usize),
    Dig(usize, usize),
}
