extern crate rand;

pub mod dreidel;
pub mod game;
pub mod io;
pub mod player;

use dreidel::Dreidel;
use player::Player;

pub trait IOProvider<T = Self>
where
    T: IOProvider,
{
    fn announce_ante(&self, &Player, usize);
    fn announce_no_qualified_player(&self);
    fn announce_turn(&self, &Dreidel, &Player, usize);
    fn announce_winner(&self, &str);
    fn set_up_game(self) -> game::Game<T>;
}
