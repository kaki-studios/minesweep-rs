mod minesweeper;
mod solver;
use core::panic;
use std::{
    env::args,
    io::{stdin, BufRead},
    process::exit,
};

use minesweeper::Minesweeper;
use rand::Rng;

use crate::minesweeper::MoveType;

pub const BOARD_WIDTH: usize = 15;
pub const BOARD_HEIGHT: usize = 15;
pub const MINE_COUNT: usize = 30;
pub const CLEAR_CODE: &'static str = "\x1B[2J\x1B[1;1H";

const USAGE: &'static str = "
USAGE:
minesweep-rs [COMMAND]
possible commands:
p (to play minesweeper yourself),
b (to watch a bot play)
";

const GUIDE: &'static str = "input must be of form: `m y,x` where m is the movetype ([f]lag or [d]ig) and y and x are coordinates";

fn main() {
    let is_bot = match args().nth(1).expect(USAGE) {
        p if p == String::from("p") => false,
        b if b == String::from("b") => true,

        m => {
            eprintln!("unknown command {m}");
            eprintln!("{USAGE}");
            exit(1);
        }
    };
    if is_bot {
        play_bot();
    } else {
        play_normal();
    }
}

fn print_empty_board() {
    print!("{}", CLEAR_CODE);
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
    for _i in 0..BOARD_HEIGHT {
        if _i < 10 {
            print!("{_i}  ");
        } else {
            print!("{_i} ");
        }
        for _j in 0..BOARD_WIDTH {
            print!("|   ");
        }
        println!("| ");
        println!("{line}");
    }
}

fn play_normal() {
    print_empty_board();
    println!("enter coords of where you want to start:");
    println!("example: `10,9`");
    let mut buffer = String::new();
    stdin().lock().read_line(&mut buffer).unwrap();
    let guess: (usize, usize) = buffer
        .split_once(",")
        .map(|(y, x)| {
            (
                y.trim().parse::<usize>().unwrap(),
                x.trim().parse::<usize>().unwrap(),
            )
        })
        .expect("input must be of form: `y,x`");
    let mut ms = Minesweeper::new_with_guess(guess);
    //main game loop
    loop {
        buffer.clear();
        ms.print_board();
        println!("possible actions:");
        println!("`f 10,9` to flag/unflag 10, 9");
        println!("`d 10,9` to dig 10, 9");

        stdin().lock().read_line(&mut buffer).unwrap();

        let (movetype, coords_str) = buffer.split_once(" ").expect(GUIDE);
        let coords = coords_str
            .split_once(",")
            .map(|(y, x)| {
                (
                    y.trim().parse::<usize>().unwrap(),
                    x.trim().parse::<usize>().unwrap(),
                )
            })
            .expect(GUIDE);
        let turn = match (movetype, coords) {
            ("f", n) => MoveType::Flag(n.0, n.1),
            ("d", n) => MoveType::Dig(n.0, n.1),
            other => panic!("invalid input: {:?}", other),
        };
        ms.play_turn(turn);
    }
}

fn play_bot() {
    print_empty_board();
    println!("type `n` to see next turn, anything else to exit");
    let mut buffer = String::new();
    stdin().lock().read_line(&mut buffer).unwrap();

    if buffer.trim() != "n" {
        return;
    }
    let mut rng = rand::thread_rng();
    //random initial guess, doesn't matter
    let initial_guess: (usize, usize) = (
        rng.gen_range(0..BOARD_HEIGHT),
        rng.gen_range(0..BOARD_WIDTH),
    );
    let mut ms = Minesweeper::new_with_guess(initial_guess);
    //main loop
    loop {
        ms.print_board();
        buffer.clear();
        println!("type `n` to see next turn, anything else to exit");
        stdin().lock().read_line(&mut buffer).unwrap();
        if buffer.trim() != "n" {
            break;
        }
        ms.play_turn(crate::solver::play(&ms));
    }
}
