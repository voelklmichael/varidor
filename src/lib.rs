#[macro_use]
extern crate lazy_static;

mod javascript_interaction;
pub use self::javascript_interaction::*;
mod game_logic;
pub use self::game_logic::*;

use std::sync::Mutex;

const BOARDSIZE: u64 = 5;

lazy_static! {
  static ref DATA: Mutex<Board> = Mutex::new(Board::new());
}
