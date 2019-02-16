#![feature(int_to_from_bytes)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate quick_error;

extern crate rand;

extern crate game_engine;
#[macro_use]
extern crate game_engine_derive;

use serde_json::Value;
use std::error::Error;
use game_engine::{*, Game as InnerGame};
use game_engine_derive::{flow, moves};
use rand::{Rng, SeedableRng, ChaChaRng};

/// Error types.
quick_error! {
    #[derive(Debug)]
    pub enum Errors {
        InvalidTile {
            description("invalid tile")
            display("A move must specify a valid tile.")
        }
    }
}

/// Define the state shape.
/// State types and enums.

// A ship's ID is its index in this array, and its length is the value at that index.
static SHIPS: [i8; 7] = [2, 2, 2, 3, 3, 4, 5];
const BOARD_SIZE: usize = 10;
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum Tile {
    Water,
    Ship(u8),
    Hit(u8),
    Miss
}

pub type Board = [[Tile; BOARD_SIZE]; BOARD_SIZE];
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub boards: Vec<Board>,
    pub hits_left: Vec<[i8; 7]>
}
impl Default for State {
    fn default() -> Self {
        State {
            boards: vec![empty_board(), empty_board()],
            hits_left: vec![SHIPS, SHIPS]
        }
    }
}

fn is_victory (state: &State) -> Option<u16> {
    for player_idx in 0..2 {
        let mut no_ships = true;
        for ship_idx in 0..SHIPS.len() {
            if state.hits_left[player_idx][ship_idx] > 0 {
                no_ships = false;
                break;
            }
        }
        if no_ships {
            // If this player doesn't have any ships remaining, the other player wins.
            return match player_idx {
                0 => Some(2),
                1 => Some(1),
                _ => None
            };
        }
    }
    None
}

fn empty_board() -> Board {
    [[Tile::Water; BOARD_SIZE]; BOARD_SIZE]
}

fn random_board (rng: &mut ChaChaRng) -> Board {
    let board = &mut empty_board();
    let board_size = BOARD_SIZE as i8;

    // Iterate over the ships by length in descending order.
    // For each ship, pick a random position/orientation until the placement is valid.
    // (Valid means the ship is surrounded by water).
    for ship_id in (0..SHIPS.len()).rev() {
        let ship_length = SHIPS[ship_id];
        let ship_id = ship_id as u8;
        loop {
            let orientation = rng.gen_bool(0.5);
            let first: i8 = rng.gen_range(0, board_size - ship_length + 1);
            let second: i8 = rng.gen_range(0, board_size);
            if (orientation && first + ship_length > board_size) ||
               (!orientation && second + ship_length > board_size) {
               continue;
            }

            let mut coords: Vec<(u8, u8)> = vec![];
            for tile in 0..ship_length {
               let tile = tile as i8;
               let coord: (i8, i8) = match orientation {
                   true => (first + tile, second),
                   _ => (first, second + tile)
               };
               let mut valid = true;
               for x_offset in 0..=2 {
                   for y_offset in 0..=2 {
                       let x = coord.0 + x_offset - 1;
                       let y = coord.1 + y_offset - 1;
                       valid = valid && ((x < 0 || y < 0) || (x >= board_size || y >= board_size) ||
                         match board[x as usize][y as usize] {
                           Tile::Ship(id) if id != ship_id => false,
                           _ => true
                       });
                   }
               }
               if valid {
                   coords.push((coord.0 as u8, coord.1 as u8));
               }
            }

            if coords.len() as i8 == ship_length {
                for coord in coords.iter() {
                    board[coord.0 as usize][coord.1 as usize] = Tile::Ship(ship_id);
                }
                break;
            }
        }
    }
    *board
}

/// Define your moves as methods in this trait.
#[moves]
trait Moves {
    fn click_tile(state: &mut UserState<State>, args: &Option<Value>)
                -> Result<(), Box<Error>> {
        let value = args.clone().ok_or_else(|| Box::new(Errors::InvalidTile))?;

        let coords: Vec<u64> = value.as_array()
            .and_then(|arr| Some(arr.iter().filter_map(|coord| coord.as_u64())))
            .ok_or(Box::new(Errors::InvalidTile))?
            .collect();

        // Ensure that the coordinates are valid.
        if coords.len() != 2 {
            return Err(Box::new(Errors::InvalidTile));
        }
        let x = coords[0] as usize;
        let y = coords[1] as usize;
        if  x > 9 || y > 9 {
            return Err(Box::new(Errors::InvalidTile));
        }

        // The current player is attacking the other player's board.
        let player_idx: usize = match state.ctx.current_player {
            1 => 1,
            2 => 0,
            _ => panic!("Invalid player")
        };
        let board = &mut state.g.boards[player_idx];
        let hits_left = &mut state.g.hits_left[player_idx];

        match board[x][y] {
            Tile::Water => {
                board[x][y] = Tile::Miss;
                Ok(())
            },
            Tile::Ship(ship_id) => {
                hits_left[ship_id as usize] -= 1;
                board[x][y] = Tile::Hit(ship_id);
                Ok(())
            },
            _ => Err(Box::new(Errors::InvalidTile))
        }
    }
}

/// Define the game flow.
#[flow]
trait Flow {
    fn initial_state(&self) -> State {
        // Generate random ship placements for each player.
        let seed = unsafe { get_seed() };
        let mut seed_arr = [0 as u8; 32];
        for (i, byte) in seed.to_le_bytes().iter().enumerate() {
            seed_arr[i] = *byte
        };
        let mut rng = ChaChaRng::from_seed(seed_arr);
        State {
            boards: vec![random_board(&mut rng), random_board(&mut rng)],
            hits_left: vec![SHIPS, SHIPS]
        }
    }

    fn end_turn_if(&self, _: &UserState<State>) -> bool {
        // End the turn after every move.
        true
    }

    fn end_game_if(&self, state: &UserState<State>) -> Option<(Option<Score>, Value)> {
        if let Some(winner) = is_victory(&state.g) {
            return Some((Some(Score::Win(winner)), json!({
                "winner": winner,
            })));
        }
        None
    }

    fn player_filter(&self) -> Option<fn(&State, u16) -> State> {
        Some(|state, player_id| {
            let boards = state.boards.iter().enumerate().map(|(idx, board)| {
                if idx as u16 != (player_id - 1) {
                    // Remove any Ship values from other players' boards.
                    let mut filtered_board: Board = empty_board();
                    for x in 0..BOARD_SIZE {
                        for y in 0..BOARD_SIZE {
                            let tile = board[x][y];
                            let cell_value = match tile {
                                Tile::Ship(ship_id) => Tile::Water,
                                Tile::Hit(ship_id) => {
                                    match state.hits_left[idx][ship_id as usize] {
                                        0 => Tile::Ship(ship_id),
                                        _ => Tile::Hit(ship_id)
                                    }
                                },
                                _ => tile
                            };
                            filtered_board[x][y] = cell_value;
                        }
                    }
                    return filtered_board;
                }
                board.clone()
            }).collect::<Vec<Board>>();
            let hits_left = state.hits_left.iter().enumerate().map(|(idx, hits)| {
                if idx as u16 == (player_id - 1) {
                    return *hits
                }
                return [0; 7];
            }).collect::<Vec<[i8; 7]>>();
            State {
                boards,
                hits_left
            }
        })
    }

    fn optimistic_update(&self, state: &UserState<State>, _: &Move) -> bool {
        // Since this game involves random state, we cannot do any optimistic updating.
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
