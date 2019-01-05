#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate quick_error;
extern crate rand_core;
extern crate rand_xorshift;

extern crate game_engine;
#[macro_use]
extern crate game_engine_derive;

use serde_json::Value;
use std::error::Error;
use game_engine::{*, Game as InnerGame};
use game_engine_derive::{flow, moves};
use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;

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
static SHIPS: [u8; 7] = [1, 1, 2, 2, 3, 4, 5];
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum Tile {
    Water,
    Ship(u8),
    Hit(u8),
    Miss
}

pub type Board = [[Tile; 10]; 10];
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub boards: [Board; 2],
    pub hits_left: [[u8; 7]; 2]
}

fn is_victory (state: State) -> Option<u16> {
    for player_idx in 0..2 {
        let no_ships = true;
        for ship_idx in 0..7 {
            if state.hits_left[player_idx][ship_idx] > 0 {
                no_ships = false;
                break;
            }
        }
        if no_ships {
            // If this player doesn't have any ships remaining, the other player wins.
            return match player_idx {
                0 => Some(2),
                1 => Some(1)
            };
        }
    }
    None
}

fn random_board (rng: &XorShiftRng) -> Board {
    [[Tile::Water; 10]; 10]
}

/// Define your moves as methods in this trait.
#[moves]
trait Moves {
    fn click_tile(state: &mut UserState<State>, args: &Option<Value>)
                -> Result<(), Box<Error>> {
        if let None = args {
            return Err(Box::new(Errors::InvalidTile));
        }
        let value = args.unwrap();

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
        if x < 0 || x > 9 {
            return Err(Box::new(Errors::InvalidTile));
        }
        if y < 0 || y > 9 {
            return Err(Box::new(Errors::InvalidTile));
        }

        // The current player is attacking the other player's board.
        let player_idx = !(state.ctx.current_player - 1) as usize;
        let mut board = &state.g.boards[player_idx];
        let mut hits_left = &state.g.hits_left[player_idx];

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
        let seed = self.seed.unwrap();
        let rng = XorShiftRng::from_seed(seed.to_le_bytes());
        State {
            boards: [random_board(&rng), random_board(&rng)],
            hits_left: [SHIPS, SHIPS]
        }
    }

    fn end_turn_if(&self, _: &UserState<State>) -> bool {
        // End the turn after every move.
        true
    }

    fn end_game_if(&self, state: &UserState<State>) -> Option<(Option<Score>, Value)> {
        if let Some(winner) = is_victory(state.g) {
            return Some((Some(Score::Win(winner)), json!({
                "winner": winner,
            })));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
