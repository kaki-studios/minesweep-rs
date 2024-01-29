use rand::Rng;

use crate::{minesweeper::*, BOARD_HEIGHT, BOARD_WIDTH};

pub fn play(ms: &Minesweeper) -> MoveType {
    let mut number_tiles = vec![];
    for (y, row) in ms.board.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Tile::Known(_) = tile {
                number_tiles.push((y, x));
            }
        }
    }
    //dig loop
    //if adjacent flags == tile num:
    //dig the rest!
    for tile in &number_tiles {
        let curr_tile = ms.board[tile.0][tile.1];
        let adjacent_tiles = get_adjacent_tiles(ms, *tile);
        let adjacent_tiles = adjacent_tiles
            .iter()
            .flatten()
            .map(|a| *a)
            .collect::<Vec<((usize, usize), Tile)>>();
        let flags = adjacent_tiles
            .iter()
            .filter(|n| n.1 == Tile::Unknown(true))
            .map(|n| n.0)
            .collect::<Vec<(usize, usize)>>();
        for (coords, tile) in adjacent_tiles {
            if let Tile::Known(val) = curr_tile {
                if val as usize == flags.len()
                    && !flags.contains(&coords)
                    && tile == Tile::Unknown(false)
                {
                    return MoveType::Dig(coords.0, coords.1);
                }
            }
        }
    }
    //flagging loop
    //if adjacent unknown tiles == tile num
    //flag those tiles
    for tile in &number_tiles {
        let curr_tile = ms.board[tile.0][tile.1];
        let adjacent_tiles = get_adjacent_tiles(ms, *tile);
        let adjacent_unknowns: Vec<(usize, usize)> = adjacent_tiles
            .iter()
            .flatten()
            .filter(|n| n.1 == Tile::Unknown(false))
            .map(|tile| tile.0)
            .collect();
        let adjacent_tiles = adjacent_tiles
            .iter()
            .flatten()
            .map(|a| *a)
            .collect::<Vec<((usize, usize), Tile)>>();
        let flags = adjacent_tiles
            .iter()
            .filter(|n| n.1 == Tile::Unknown(true))
            .map(|n| n.0)
            .collect::<Vec<(usize, usize)>>();
        if let Tile::Known(val) = curr_tile {
            if adjacent_unknowns.len() == (val as usize - flags.len())
                && adjacent_unknowns.len() > 0
            {
                return MoveType::Flag(adjacent_unknowns[0].0, adjacent_unknowns[0].1);
            }
        }
    }
    //nothing to do, most probably a situation where you have to guess, so we'll guess
    guess(ms)
}

fn guess(ms: &Minesweeper) -> MoveType {
    let mut unknowns = vec![];
    for (y, row) in ms.board.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Tile::Unknown(false) = tile {
                unknowns.push((y, x));
            }
        }
    }
    let mut rng = rand::thread_rng();

    let tile = if unknowns.len() > 0 {
        unknowns[rng.gen_range(0..unknowns.len())]
    } else {
        (
            rng.gen_range(0..BOARD_HEIGHT),
            rng.gen_range(0..BOARD_WIDTH),
        )
    };
    MoveType::Dig(tile.0, tile.1)
}

fn get_adjacent_tiles(
    ms: &Minesweeper,
    coords: (usize, usize),
) -> [Option<((usize, usize), Tile)>; 8] {
    let mut list: [Option<((usize, usize), Tile)>; 8] = [None; 8];
    let coords = (coords.0 as i32, coords.1 as i32);
    for (i, adj) in ADJACENTS.iter().enumerate() {
        let adj_coord = (coords.0 + adj.0, coords.1 + adj.1);
        if (0..BOARD_HEIGHT).contains(&(adj_coord.0 as usize))
            && (0..BOARD_WIDTH).contains(&(adj_coord.1 as usize))
        {
            list[i] = Some((
                (adj_coord.0 as usize, adj_coord.1 as usize),
                ms.board[adj_coord.0 as usize][adj_coord.1 as usize],
            ));
        }
    }
    list
}
